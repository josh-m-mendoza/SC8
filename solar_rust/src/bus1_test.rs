mod parser;
use parser::parse_log;
mod messages_kelly;
// use messages_kelly;
mod decode;
use anyhow::{Context, Result};
use socketcan::{dump::Reader, CanAnyFrame, CanFrame,CanFdSocket, Socket};
use std::process;
use std::path::Path;
use embedded_can::Frame;
mod compact;
use compact::compact_data;
use CanAnyFrame::*;

fn bus1_test() -> anyhow::Result<()> {

    compact_data(); // Helper function to make compact candump data: Only used for the mock data that we got from the Car

    let sock = CanFdSocket::open("vcan0") // Open Socket Connection. Using virtual can now for testing. This socket is what we will write to
        .with_context(|| format!("Failed to open socket on vcan"))?;

    let path = Path::new("kelly_kls_test_data.txt"); // Path to test data

    let sock_rx = CanFdSocket::open("vcan0")  // Second Virtual Can Socket Connection. This socket will read from the socket we are writing to
        .with_context(|| format!("Failed to oped rx socket on vcan0"))?;

    let mut reader = Reader::from_file(&path)  // Use rust socketcan dump reader
        .with_context(|| format!("Error opening log file"))?;

    for rec in reader.records(){ //Loop through each line in candumps file. 
    

        let (ts,frame) = rec?; // seperate by timestamp and actuale frame
        println!("{:?}",frame);

        match frame {  // match based on what type of frame we have ie normal data or flexible data. Once matched, write to 1st socket
            CanAnyFrame::Normal(f) => sock.write_frame(&f)?,
            CanAnyFrame::Remote(f) => sock.write_frame(&f)?,
            CanAnyFrame::Fd(f)     => sock.write_frame(&f)?,
            _ => {}
        }


        // Read frame after we write to do computation on it
        let read_frame = sock_rx.read_frame().context("Recieving Frame")?;


        //Call helper function from dbc codegen file. from_can_message matches id of frame to either Message 1 or Message 2
        let matched_frame = messages_kelly::Messages::from_can_message(read_frame.id(),read_frame.data()); 
        
        println!("{:?}", matched_frame);
        
        match matched_frame{

            Ok(messages_kelly::Messages::Message1(msg)) => {  //Message 1 match chase
                let translated = messages_kelly::Message1::decodeMessage(&msg); // Call helper function to decode Message 1 Type

                println!("{:?}", translated);
            }

            Ok(messages_kelly::Messages::Message2(msg)) => { //Message 2 match case
                let translated = messages_kelly::Message2::decodeMessage(&msg); // Call helper function to decode Message 2 Type

                println!("{:?}",translated);
            }
            
            Err(_) => { println!("Cant Decode this shit");
        }
        }


    }
    
    Ok(())
}

