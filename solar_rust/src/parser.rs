use std::fs::File;
use std::io::{BufRead, BufReader};
use embedded_can::Frame;
use socketcan::{CanFrame, CanId};
use std::convert::TryFrom;

fn make_frame(id_raw: u32, data: [u8; 8]) -> CanFrame {
    let id = CanId::try_from(id_raw).expect("Invalid CAN ID");
    CanFrame::new(id, &data).expect("Invalid CAN frame")
}

pub fn parse_log(path: &str) -> Vec<CanFrame> {
    let file = File::open(path).expect("Could not open log file");
    let reader = BufReader::new(file);
    let mut frames = Vec::new();

    for line in reader.lines().skip(1) { // skip header
        let line = line.unwrap();
        if line.trim().is_empty() { continue; }

        // Split on commas, and trim spaces
        let cols: Vec<_> = line.split(',')
            .map(|s| s.trim())
            .collect();

        if cols.len() < 6 { continue; }

        let id_str = cols[3];   // e.g. "0x502"
        let data_str = cols[5]; // e.g. "0x3f80000000000000"

        // Parse ID
        let id = u32::from_str_radix(id_str.trim_start_matches("0x"), 16)
            .expect("Invalid CAN ID");

        // Parse data into bytes
        let mut hex_data = data_str.trim_start_matches("0x").to_string();
        if hex_data.len() % 2 != 0 {
            hex_data = format!("0{hex_data}");
        }

        let data = (0..hex_data.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_data[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();

        let array_data: [u8; 8] = data.try_into().expect("Expected exactly 8 bytes");

        frames.push(make_frame(id, array_data));
    }

    frames
}