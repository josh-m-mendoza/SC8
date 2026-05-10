mod messages_kelly;
use CanAnyFrame::*;
use anyhow::{Context, Result};
use embedded_can::Frame;
use redis::AsyncCommands;
use socketcan::{CanAnyFrame, CanFdSocket, CanFrame, Socket, dump::Reader};
use solar_rust::bus1::connect_bus_1;
use solar_rust::bus2::connect_bus_2;
use std::path::Path;
use std::process;
use tokio::task::spawn;
use tracing_subscriber;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let connection = "redis://127.0.0.1";
    let client = redis::Client::open(connection)?;
    let mut con: redis::aio::MultiplexedConnection =
        client.get_multiplexed_async_connection().await?; // Make connection to redis. Its multiplexed so imma pass clones to each worker  thread but imma do it later
    tracing_subscriber::fmt::init(); // initialize logger

    let mut handles = vec![];
    let kelly_handle = tokio::task::spawn(async { connect_bus_1().await }); //spawn tokio async threads. 
    let mppt_handle = tokio::task::spawn(async { connect_bus_2().await });

    handles.push(kelly_handle);
    handles.push(mppt_handle);

    for handle in handles {
        //run threads
        handle.await.unwrap();
    }

    Ok(())
}
