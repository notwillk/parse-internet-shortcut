mod parser;

use std::{
    fs,
    io::{Read, Write},
    process::ExitCode,
};

use clap::Parser;
use parser::{ParseError, parse_shortcut};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "parse-internet-shortcut")]
#[command(about = "Parse Internet Shortcut files and print JSON")]
struct Args {
    path: Option<String>,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("missing input path")]
    MissingInput,
    #[error("failed to read file: {path}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read stdin")]
    ReadStdin(#[source] std::io::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("failed to write JSON output")]
    Serialize(#[source] serde_json::Error),
    #[error("failed to write output newline")]
    WriteNewline(#[source] std::io::Error),
}

impl AppError {
    fn exit_code(&self) -> ExitCode {
        match self {
            AppError::MissingInput => ExitCode::from(1),
            AppError::ReadFile { .. } | AppError::ReadStdin(_) => ExitCode::from(2),
            AppError::Parse(_) => ExitCode::from(3),
            AppError::Serialize(_) | AppError::WriteNewline(_) => ExitCode::from(4),
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            error.exit_code()
        }
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();
    let input = args.path.ok_or(AppError::MissingInput)?;

    let content = if input == "-" {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map_err(AppError::ReadStdin)?;
        buffer
    } else {
        fs::read_to_string(&input).map_err(|source| AppError::ReadFile {
            path: input.clone(),
            source,
        })?
    };

    let parsed = parse_shortcut(&content)?;
    let mut stdout = std::io::stdout();
    serde_json::to_writer_pretty(&mut stdout, &parsed).map_err(AppError::Serialize)?;
    stdout.write_all(b"\n").map_err(AppError::WriteNewline)?;

    Ok(())
}
