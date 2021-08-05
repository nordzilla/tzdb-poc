//! A very primitive proof of concept for building and reading the IANA TZDB from Rust.
//! This just compiles the C binaries and runs them directly.

use clap::{App, Arg};
use std::process::Command;

const RELATIVE_TZDB_PATH: &str = "./tzdb-2021a";
const RELATIVE_DATA_PATH: &str = "../tzdata";

/// Makes the TZDB binaries.
fn make_binaries() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("make")
        .current_dir(RELATIVE_TZDB_PATH)
        .arg("-s")
        .status()?;
    Ok(())
}

/// Compile TZDB into tzif files using zic.
fn compile_data() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("./zic")
        .current_dir(RELATIVE_TZDB_PATH)
        .arg("africa")
        .arg("antarctica")
        .arg("asia")
        .arg("australasia")
        .arg("etcetera")
        .arg("europe")
        .arg("northamerica")
        .arg("southamerica")
        .arg("-d")
        .arg(RELATIVE_DATA_PATH)
        .status()?;
    Ok(())
}

/// Dump time-zone data for a particular year using zdump.
/// If no year is present, return data for all relevant years.
fn zdump(tzid: &str, year: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
    let mut zdump = Command::new("./zdump");
    zdump.current_dir(RELATIVE_TZDB_PATH);
    zdump.arg("-V");
    zdump.arg(tzid);
    if let Some(year) = year {
        zdump.arg("-c");
        zdump.arg(&format!("{},{}", year, year + 1));
    }
    zdump.status()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("DTS Transition Lookup")
        .version("0.0.0")
        .about("A proof-of-concept tool to build the IANA TZDB and look up daylight-savings-time transitions")
        .arg(
            Arg::with_name("tzid")
                .index(1)
                .value_name("TZID")
                .required(true),
        )
        .arg(
            Arg::with_name("year")
                .index(2)
                .value_name("YEAR")
                .required(false),
        )
        .get_matches();

    let tzid = matches.value_of("tzid").unwrap();
    let year = matches
        .value_of("year")
        .map(|year| year.parse::<i32>().unwrap());

    make_binaries()?;
    compile_data()?;
    zdump(tzid, year)?;

    Ok(())
}
