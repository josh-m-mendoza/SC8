use embedded_can::Frame;
use socketcan::{CanFrame, CanId};
use solar_rust::parser::parse_log;
use solar_rust::messages_motorcontroller;

#[test]
#[cfg(feature = "debug")]
fn golden_decode_motorcontroller() {
    let frames = parse_log("src/Mock_Kelly_Data.txt");
    assert!(!frames.is_empty());

    for parsed in frames {
        println!("frame: id={:?} data={:?}", parsed.id(), parsed.data());

        let decoded = messages_motorcontroller::Messages::from_can_message(parsed.id(), parsed.data());

        match decoded {
            Ok(msg) => {
                println!("decoded: {:?}", msg);
            }
            Err(e) => {
                // println!("decode error: {:?}", e);
            }
        }
    }
}
