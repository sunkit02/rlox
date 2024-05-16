use anyhow::{Context, Result};
use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process,
    str::FromStr,
};

use crate::lexer::Lexer;

mod lexer;

fn main() -> Result<()> {
    // Skip the current exe name
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        let path = PathBuf::from_str(&args[0]).context("convert String to PathBuf")?;
        run_file(path)?;
    } else {
        run_prompt()?;
    };

    Ok(())
}

fn run_file(path: PathBuf) -> Result<()> {
    let src_file = fs::read_to_string(path)?;

    run(src_file.as_str());

    Ok(())
}

fn run_prompt() -> Result<()> {
    let prompt: &str = "> ";

    print!("{}", prompt);
    stdout().lock().flush().context("flush stdout")?;
    for line in stdin().lines() {
        let line = line.context("read line from stdin")?;
        run(line.as_str());

        print!("{}", prompt);
        stdout().lock().flush().context("flush stdout")?;
    }

    Ok(())
}

fn run(source: &str) {
    let lexer = Lexer::new(&source);
    for result in lexer.scan_all_tokens() {
        match result {
            Ok(token) => println!("{token}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
