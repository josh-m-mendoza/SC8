use crate::messages_kelly;
use crate::messages_kelly::Message2FeedbackStatus;
use anyhow::{Context, Result};
use embedded_can::Frame;
use iovec::IoVec;
use messages_kelly::Message2CommandStatus;
use nix::cmsg_space;
use nix::sys::socket::RecvMsg;
use nix::sys::socket::{ControlMessageOwned, MsgFlags, recvmsg, setsockopt, sockopt};
use nix::sys::time::TimeSpec;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use redis::cmd;
use redis_ts::TsDuplicatePolicy;
use redis_ts::TsOptions;
use socketcan::{
    CanAnyFrame, CanFdSocket, CanFrame, CanSocket, ExtendedId, Id, Socket, StandardId, dump::Reader,
};
use std::fmt;
use std::future::Future;
use std::io::IoSliceMut;
use std::os::fd::{AsRawFd, RawFd};
use tokio::io::Interest;
use tokio::io::unix::AsyncFd;
use tracing::{error, info};
use tracing_subscriber;

pub async fn connect_bus_1() -> anyhow::Result<()> {
    let connection = "redis://127.0.0.1";
    let client = redis::Client::open(connection)?;
    let mut con: redis::aio::MultiplexedConnection =
        client.get_multiplexed_async_connection().await?;
    let sock_rx = match socketcan::CanSocket::open("vcan1") {
        //Open socket connection
        Ok(s) => {
            info!("Succesfully connected to Kelly  bus");
            s
        }
        Err(e) => {
            error!("Failed to make socket connection:{}", e);
            return Err(e).context("Failed to connect to the KellyKLS bus");
        }
    };

    let keys = [
        //instantiate redis keys
        "Kelly:motor_current",
        "Kelly:battery_voltage",
        "Kelly:id_error",
        "Kelly:over_voltage",
        "Kelly:low_voltage",
        "Kelly:stall",
        "Kelly:internal_volts_fault",
        "Kelly:over_temperature",
        "Kelly:throttle_error",
        "Kelly:hall_throttle_open",
        "Kelly:angle_sensor_error",
        "Kelly:motor_over_temperature",
        "Kelly:hall_galvanometer_error",
        "Kelly:throttle_signal",
        "Kelly:controller_temperature",
        "Kelly:command_status",
        "Kelly:feedback_status",
        "Kelly:hall_a",
        "Kelly:hall_b",
        "Kelly:hall_c",
        "Kelly:brake_switch",
        "Kelly:backward_switch",
        "Kelly:forward_switch",
        "Kelly:foot_switch",
        "Kelly:boost_switch",
    ];

    for key in keys {
        //Data for Each key persists for 1 min
        let _: Result<(), redis::RedisError> = cmd("TS.CREATE")
            .arg(key)
            .arg("RETENTION")
            .arg(60_000)
            .arg("DUPLICATE_POLICY")
            .arg("last")
            .query_async(&mut con)
            .await;
    }
    sock_rx.set_nonblocking(true)?; // set socket to nonblocking for async fd

    let timestamp = true; //enable timestamps on socket reads so we can get timestamps with message

    match setsockopt(
        &sock_rx,
        nix::sys::socket::sockopt::ReceiveTimestamp,
        &timestamp,
    ) {
        //use nix setsockopt to enable timestamp
        Ok(_) => {}
        Err(e) => {
            error!("Nix error during setsock, {}", e);
        }
    }

    let async_fd = AsyncFd::new(sock_rx)?; //wrap socket with tokio asyncfd to get async capabilities when reading

    let flags = MsgFlags::empty(); //no flags so we pass empty

    loop {
        let (iov, time) = async_fd
            .async_io(Interest::READABLE, |inner| {
                //Calling async_io method. 2 paramaters, we are reading so Interest readable. Second paramater is a closure, Inner is a mut reference  to the socket thats being wrapped by async fd
                let mut data = [0u8; 16]; // data that we will be recieving from socket
                let time = {
                    let mut cmsg_buffer = cmsg_space!(nix::sys::time::TimeVal); // cmsg buffer is what timestamp gets written into

                    let mut iov = [IoSliceMut::new(&mut data)]; // wrap data in iovector
                    let r =
                        recvmsg::<()>(inner.as_raw_fd(), &mut iov, Some(&mut cmsg_buffer), flags)?; //call nix  recvmsg to get message from socket. Pass in raw  filedescriptor, iovector,  flags, and cmsg  buffer. 
                    let time = match r.cmsgs()?.next() {
                        Some(ControlMessageOwned::ScmTimestamp(rtime)) => Some(rtime),
                        _ => None,
                    };
                    time // extract time from cmsg buffer
                };

                let mut data_clone: [u8; 16] = [0u8; 16];

                data_clone.clone_from_slice(&data); //clone data, we  need  to return owned array from closure

                Ok((data_clone, time)) // pass up data clone and  time
            })
            .await?;

        let time_seconds = match time {
            //ectract the official time value, if it was None we use the redis default value  "*" where redis provides the time
            Some(t_value) => t_value.tv_sec().to_string(),
            None => {
                error!("Timestamp not written into cmsg buffer, using redis default value, *");
                "*".to_string()
            }
        };
        println!("{}", time_seconds);
        let can_id: u32 =
            iov[0] as u32 | (iov[1] as u32) << 8 | (iov[2] as u32) << 16 | (iov[3] as u32) << 24;

        let eff_flag = (iov[3] as u32 & 0x80 as u32) >> 7;

        let clean_id = if eff_flag == 1 {
            match socketcan::ExtendedId::new(can_id & 0x1FFFFFFF) {
                Some(id) => socketcan::Id::Extended(id),
                None => {
                    error!(eff_flag = true, "Invalid extended CAN ID");
                    continue;
                }
            }
        } else {
            match socketcan::StandardId::new((can_id & 0x7FF) as u16) {
                Some(id) => socketcan::Id::Standard(id),
                None => {
                    error!(eff_flag = false, "Invalid standard CAN ID");
                    continue;
                }
            }
        };

        let frame_data: &[u8] = &iov[8..=15];

        let can_frame = match socketcan::frame::CanFrame::new(clean_id, frame_data) {
            Some(frame) => frame,
            None => {
                error!(id = ?clean_id, data = frame_data, "Failed to cast Id and data into CAN Frame");
                continue;
            }
        };

        let matched_frame =
            messages_kelly::Messages::from_can_message(can_frame.id(), can_frame.data());

        match matched_frame {
            Ok(messages_kelly::Messages::Message1(frame)) => {
                if let Err(e) = redis::cmd("TS.MADD")
                    .arg("Kelly:motor_current")
                    .arg(&time_seconds)
                    .arg(frame.motor_current())
                    .arg("Kelly:battery_voltage")
                    .arg(&time_seconds)
                    .arg(frame.battery_voltage())
                    .arg("Kelly:id_error")
                    .arg(&time_seconds)
                    .arg(frame.id_error())
                    .arg("Kelly:over_voltage")
                    .arg(&time_seconds)
                    .arg(frame.over_voltage())
                    .arg("Kelly:low_voltage")
                    .arg(&time_seconds)
                    .arg(frame.low_voltage())
                    .arg("Kelly:stall")
                    .arg(&time_seconds)
                    .arg(frame.stall())
                    .arg("Kelly:internal_volts_fault")
                    .arg(&time_seconds)
                    .arg(frame.internal_volts_fault())
                    .arg("Kelly:over_temperature")
                    .arg(&time_seconds)
                    .arg(frame.over_temperature())
                    .arg("Kelly:throttle_error")
                    .arg(&time_seconds)
                    .arg(frame.throttle_error())
                    .arg("Kelly:hall_throttle_open")
                    .arg(&time_seconds)
                    .arg(frame.hall_throttle_open())
                    .arg("Kelly:angle_sensor_error")
                    .arg(&time_seconds)
                    .arg(frame.angle_sensor_error())
                    .arg("Kelly:motor_over_temperature")
                    .arg(&time_seconds)
                    .arg(frame.motor_over_temperature())
                    .arg("Kelly:hall_galvanometer_error")
                    .arg(&time_seconds)
                    .arg(frame.hall_galvanometer_error())
                    .query_async::<_, Vec<u64>>(&mut con)
                    .await
                {
                    error!(frame = ?frame, error = %e, "Redis Write on Message1 Frame Failed");
                };

                info!(frame = ?frame, time = &time_seconds, "Successfully Wrote PowerInput Frame");
            }

            Ok(messages_kelly::Messages::Message2(frame)) => {
                let command_status: u8 = match frame.command_status() {
                    Message2CommandStatus::Backward => {
                        0 //Backward
                    }
                    Message2CommandStatus::Forward => {
                        1 //Forward
                    }
                    Message2CommandStatus::Neutral => {
                        2 // Neutral
                    }
                    Message2CommandStatus::Reserved => {
                        3 // Reserved
                    }
                    _ => {
                        4 //Unknown
                    }
                };
                let feedback_status: u8 = match frame.feedback_status() {
                    Message2FeedbackStatus::Stationary => {
                        0 // Stationary
                    }
                    Message2FeedbackStatus::Forward => {
                        1 // Forward
                    }
                    Message2FeedbackStatus::Backward => {
                        2 //Backward
                    }
                    Message2FeedbackStatus::Reserved => {
                        3 // Reserved
                    }
                    _ => {
                        4 // Unknown
                    }
                };

                if let Err(e) = redis::cmd("TS.MADD")
                    .arg("Kelly:throttle_signal")
                    .arg(&time_seconds)
                    .arg(frame.throttle_signal())
                    .arg("Kelly:controller_temperature")
                    .arg(&time_seconds)
                    .arg(frame.controller_temperature())
                    .arg("Kelly:motor_temperature")
                    .arg(&time_seconds)
                    .arg(frame.motor_temperature())
                    .arg("Kelly:command_status")
                    .arg(&time_seconds)
                    .arg(command_status)
                    .arg("Kelly:feedback_status")
                    .arg(&time_seconds)
                    .arg(feedback_status)
                    .arg("Kelly:hall_a")
                    .arg(&time_seconds)
                    .arg(frame.hall_a())
                    .arg("Kelly:hall_b")
                    .arg(&time_seconds)
                    .arg(frame.hall_b())
                    .arg("Kelly:hall_c")
                    .arg(&time_seconds)
                    .arg(frame.hall_c())
                    .arg("Kelly:brake_switch")
                    .arg(&time_seconds)
                    .arg(frame.brake_switch())
                    .arg("Kelly:backward_switch")
                    .arg(&time_seconds)
                    .arg(frame.backward_switch())
                    .arg("Kelly:forward_switch")
                    .arg(&time_seconds)
                    .arg(frame.forward_switch())
                    .arg("Kelly:foot_switch")
                    .arg(&time_seconds)
                    .arg(frame.foot_switch())
                    .arg("Kelly:boost_switch")
                    .arg(&time_seconds)
                    .arg(frame.boost_switch())
                    .query_async::<_, Vec<u64>>(&mut con)
                    .await
                {
                    error!(frame = ?frame, error = %e, "Failed Redis Write on Message2 Frame");
                };

                info!(frame = ?frame, time = &time_seconds, "Successfully Wrote Message2 Frame");
            }

            _ => {
                error!(failed_frame = ?matched_frame ,"Cant decode this message. Expected a KellyKLS motorcontroller Frame");
            }
        }
    }
}
