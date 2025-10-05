use anyhow::{anyhow, Result};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    process::Command,
    path::Path,
};

use dbc_codegen::*;

fn main() -> Result<()> {
    // tuples of dbc files : message files
    let jobs = [
        ("./drivercontroller.dbc", "src/messages_drivercontroller.rs"),
        ("./motorcontroller.dbc", "src/messages_motorcontroller.rs"),
        ("./mppt.dbc", "src/messages_mppt.rs"),
    ];

    let mut errors: Vec<anyhow::Error> = Vec::new();

    for (dbc_path, out_path) in &jobs {
        match process_one(dbc_path, out_path) {
            Ok(_) => println!("wrote {}", out_path),
            Err(e) => {
                eprintln!("failed to process {} -> {}: {}", dbc_path, out_path, e);
                errors.push(e);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("one or more DBCs failed (see stderr)"))
    }
}

fn process_one(dbc_path: &str, out_path: &str) -> Result<()> {
    let dbc_bytes = fs::read(dbc_path)?;
    let mut out = BufWriter::new(File::create(out_path)?);

    let dbc_name = Path::new(dbc_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.dbc");

    let config = Config::builder()
        .dbc_name(dbc_name)
        .dbc_content(&dbc_bytes)
        .impl_debug(FeatureConfig::Always)
        .impl_defmt(FeatureConfig::Always)
        .impl_error(FeatureConfig::Gated("std"))
        .impl_arbitrary(FeatureConfig::Gated("arb"))
        .check_ranges(FeatureConfig::Never)
        .build();

    dbc_codegen::codegen(config, &mut out)?;
    out.flush()?;

    // run rustfmt on the generated file
    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(out_path)
        .status()?;

    if !status.success() {
        eprintln!("rustfmt failed for {} (exit {})", out_path, status);
    }

    Ok(())
}
