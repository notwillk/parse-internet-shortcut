mod parser;

use std::{
    fs,
    io::{Read, Write},
    process::ExitCode,
};

use clap::{Parser, error::ErrorKind};
use parser::{ParseError, parse_shortcut};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "parse-internet-shortcut")]
#[command(about = "Parse Internet Shortcut files and print JSON")]
struct Args {
    path: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("missing input path")]
    MissingInput,
    #[error("{0}")]
    Usage(String),
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
            AppError::MissingInput | AppError::Usage(_) => ExitCode::from(1),
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
    let args = Args::try_parse().map_err(map_clap_error)?;
    let input = args.path;

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

fn map_clap_error(error: clap::Error) -> AppError {
    if error.kind() == ErrorKind::MissingRequiredArgument {
        return AppError::MissingInput;
    }

    let raw_message = error.to_string();
    let message = raw_message
        .strip_prefix("error: ")
        .unwrap_or(&raw_message)
        .trim_end()
        .to_owned();
    AppError::Usage(message)
}
