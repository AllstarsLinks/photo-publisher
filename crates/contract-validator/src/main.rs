\
use anyhow::{bail, Result};
use photo_publisher_contract_validator::validate;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        bail!("usage: contract-validator <schema.json> <document.json>");
    }

    let schema = PathBuf::from(&args[0]);
    let document = PathBuf::from(&args[1]);

    validate(&schema, &document)?;
    println!("VALID: {}", document.display());
    Ok(())
}
