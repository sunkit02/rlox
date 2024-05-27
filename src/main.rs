use anyhow::Context;
use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process,
    str::FromStr,
};

use rlox::{
    interpreter::{ErrorReporter, Interpreter},
    lexer::{self, Lexer},
};
use rlox::{lexer::token::Token, parser::Parser};

struct StderrErrorReporter;

impl ErrorReporter for StderrErrorReporter {
    fn report_err(&self, error: &rlox::interpreter::error::RuntimeError) {
        eprintln!("{error}");
    }
}

fn main() -> anyhow::Result<()> {
    // Skip the current exe name
    let args: Vec<String> = env::args().skip(1).collect();

    let err_reporter: Box<dyn ErrorReporter> = Box::new(StderrErrorReporter);

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        let path = PathBuf::from_str(&args[0]).context("convert String to PathBuf")?;
        run_file(path)?;
    } else {
        run_prompt([err_reporter])?;
    };

    Ok(())
}

fn run_file(path: PathBuf) -> anyhow::Result<()> {
    let src_file = fs::read_to_string(path)?;

    let err_reporter: Box<dyn ErrorReporter> = Box::new(StderrErrorReporter);
    let mut interpreter = Interpreter::with_reporters([err_reporter]);

    run(src_file.as_str(), &mut interpreter)?;

    Ok(())
}

fn run_prompt<I: IntoIterator<Item = Box<dyn ErrorReporter>>>(
    err_reporter: I,
) -> anyhow::Result<()> {
    let mut interpreter = Interpreter::with_reporters(err_reporter);

    let prompt: &str = "> ";

    print!("{}", prompt);
    stdout().lock().flush().context("flush stdout")?;
    for line in stdin().lines() {
        let line = line.context("read line from stdin")?;

        if let Err(e) = run(line.as_str(), &mut interpreter) {
            eprintln!("{e}");
        }

        print!("{}", prompt);
        stdout().lock().flush().context("flush stdout")?;
    }

    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> anyhow::Result<()> {
    let lexer = Lexer::new(&source);
    let tokens = lexer
        .scan_all_tokens()
        .into_iter()
        .collect::<lexer::Result<Vec<Token>>>()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    statements.iter().for_each(|stmt| println!("{stmt}"));

    interpreter.interpret(statements);

    Ok(())
}
