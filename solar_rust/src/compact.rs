use std::fs::File;
use anyhow::{Context, Result};
use std::io::{self, BufRead, BufReader, Write};


//Helper function to take expanded frames form car test data and make compact frames so socketcan dump reader can interpret
pub fn compact_data() -> anyhow::Result<()>{

    let file_path = "bob.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut file = File::create("compact_data.txt")?;

    for line_result in reader.lines() {
        let line = line_result?;
        let data_vec: Vec<&str> = line.split(" ").collect();
        let ts = data_vec[1];
        let id = data_vec[5];
        let data = data_vec[10].to_owned() + data_vec[11] + data_vec[12] + data_vec[13] + data_vec[14] + data_vec[15] + data_vec[16] + data_vec[17];
        let id_and_data = format!("{}#{}", id,data);

        let final_line = format!("{}  {}", ts, id_and_data);
        writeln!(file, "{}", final_line)?;

    }
    Ok(())
}