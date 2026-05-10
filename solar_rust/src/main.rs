mod parser;
use parser::parse_log;
mod messages_kelly;
// use messages_kelly;
use anyhow::{Context, Result};
use socketcan::{dump::Reader, CanAnyFrame, CanFrame,CanFdSocket, Socket};
use std::process;
use std::path::Path;
use embedded_can::Frame;
use CanAnyFrame::*;
use solar_rust::bus2::connect_bus_2;
use solar_rust::bus1::connect_bus_1;
use tokio::task::spawn;
use redis::AsyncCommands;
use tracing_subscriber;


#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let connection = "redis://127.0.0.1";
    let client = redis::Client::open(connection)?;
    let mut con: redis::aio::MultiplexedConnection = client.get_multiplexed_async_connection().await?;
    tracing_subscriber::fmt::init();


    let mut handles = vec![];
    let kelly_handle = tokio::task::spawn(async{connect_bus_1().await});
    let mppt_handle = tokio::task::spawn(async {connect_bus_2().await});

    handles.push(kelly_handle); 
    handles.push(mppt_handle);


    for handle in handles{
        handle.await.unwrap();
    }

    
    Ok(())
}

