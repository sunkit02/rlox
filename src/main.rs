use anyhow::Context;
use parser::Parser;
use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process,
    str::FromStr,
};

use crate::lexer::Lexer;

mod lexer;
mod parser;

fn main() -> anyhow::Result<()> {
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

fn run_file(path: PathBuf) -> anyhow::Result<()> {
    let src_file = fs::read_to_string(path)?;

    run(src_file.as_str())?;

    Ok(())
}

fn run_prompt() -> anyhow::Result<()> {
    let prompt: &str = "> ";

    print!("{}", prompt);
    stdout().lock().flush().context("flush stdout")?;
    for line in stdin().lines() {
        let line = line.context("read line from stdin")?;

        if let Err(e) = run(line.as_str()) {
            eprintln!("{e}");
        }

        print!("{}", prompt);
        stdout().lock().flush().context("flush stdout")?;
    }

    Ok(())
}

fn run(source: &str) -> anyhow::Result<()> {
    let lexer = Lexer::new(&source);
    let tokens = lexer
        .scan_all_tokens()
        .into_iter()
        .collect::<lexer::Result<Vec<_>>>()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;

    println!("{expr}");

    Ok(())
}
