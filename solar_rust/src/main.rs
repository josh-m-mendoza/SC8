mod messages_drivercontroller;
// // mod messages_motorcontroller;
mod messages_mppt;
mod parser;
use parser::parse_log;
mod messages_kelly;
// use messages_kelly;
mod decode;
use anyhow::{Context, Result};
use clap::{arg, ArgAction, Command};
use socketcan::{dump::Reader, CanAnyFrame, CanFrame,CanFdSocket, Socket};
use std::process;
use std::path::Path;
use embedded_can::Frame;
mod compact;
use compact::compact_data;

fn main() -> anyhow::Result<()> {
    // let frames = parse_log("src/Mock_Kelly_Data.txt");
    // assert!(!frames.is_empty());
    compact_data();
    println!("nusolar is the best");
    let sock = CanFdSocket::open("vcan0")
        .with_context(|| format!("Failed to open FD socket on vcan"))?;

    let path = Path::new("compact_data.txt");
    // println!("{:?}",path);

    let sock_rx = CanFdSocket::open("vcan0")
        .with_context(|| format!("Failed to oped rx socket on can0"))?;

    let mut reader = Reader::from_file(&path)
        .with_context(|| format!("Error opening log file"))?;

    for rec in reader.records(){
    
    // loop {
        let (ts,frame) = rec?;
        println!("{:?}",frame);

        use CanAnyFrame::*;
        match frame {
            CanAnyFrame::Normal(f) => sock.write_frame(&f)?,
            CanAnyFrame::Remote(f) => sock.write_frame(&f)?,
            CanAnyFrame::Fd(f)     => sock.write_frame(&f)?,
            _ => {}
        }

        let read_frame = sock_rx.read_frame().context("Recieving Frame")?;
        // println!("{:?}", read_frame.id()); 
        // println!("{:?}", read_frame.is_standard());
        // println!("{:?}", read_frame.data()); 

        let matched_frame = messages_kelly::Messages::from_can_message(read_frame.id(),read_frame.data());
        println!("{:?}", matched_frame);
        
        match matched_frame{
            Ok(messages_kelly::Messages::Message1(msg)) => {
                let translated = messages_kelly::Message1::decodeMessage(&msg);

                println!("{:?}", translated);
            }

            Ok(messages_kelly::Messages::Message2(msg)) => {
                let translated = messages_kelly::Message2::decodeMessage(&msg);

                println!("{:?}",translated);
            }
            
            Err(_) => { println!("Cant Decode this shit");
        }
        }


    }



//     // for frame in frames{
//     //     let decoded = messages_kelly::Messages::from_can_message(frame.id(), frame.data());
//     //     // match decoded{
//     //     //     Ok(messages_kelly::Messages::Message1(msg)) =>{
//     //     //         let translated = messages_kelly::Message1::decodeMessage(&msg);
//     //     //         println!("{:?}",translated);
//     //     //     }
//     //     //     Ok(messages_kelly::Messages::Message2(msg))=>{
//     //     //         println!("Message2");
//     //     //     }
//     //     //     Err(_)=>{
//     //     //         println!("Couldnt translate");
//     //     //     }
//     //     // }
        
//     //     println!("{:?}",decoded);
//     // }
//     // println!("I'm totally not a F1 spy");
    
    Ok(())
}

