use anyhow::{Context, Result};
use std::{env, path::PathBuf, str::FromStr};

fn main() -> Result<()> {
    // Skip the file name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if args.len() == 1 {
        let path = PathBuf::from_str(&args[0]).context("convert String to PathBuf")?;
        run_file(path);
    } else {
        run_prompt();
    };

    Ok(())
}

fn run_file(path: PathBuf) {
    todo!()
}

fn run_prompt() {
    todo!()
}
