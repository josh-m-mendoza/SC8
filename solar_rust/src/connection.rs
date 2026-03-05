use socketcan::{async_std::CanSocket, CanFrame, Result};

async fn make_connection() -> Result<()> {
    let sock_read = CanSocket::open("can0")?;
    println!("Reading on can0");

    loop {
        match sock_read.read_frame().await {
            Ok(CanFrame::Data(frame)) => {
                println!("{:?}", frame)
            },
            Ok(CanFrame::Remote(frame)) => println!("{:?}", frame),
            Ok(CanFrame::Error(frame)) => println!("{:?}", frame),
            Err(err) => eprintln!("{}", err),
        }
    }
}