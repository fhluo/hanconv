use clap::{Args, Parser, Subcommand};
use hanconv::conv::{Convertor, Convertors::*};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

#[derive(Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    S2T(Args_),
    S2TW(Args_),
    S2TWP(Args_),
    T2S(Args_),
    T2TW(Args_),
    TW2S(Args_),
    TW2SP(Args_),
    TW2T(Args_),
    S2HK(Args_),
    HK2S(Args_),
    HK2T(Args_),
    T2HK(Args_),
    T2JP(Args_),
    JP2T(Args_),
}

fn convert(convertor: Convertor, args: Args_) -> io::Result<()> {
    if let Some(items) = args.items {
        for item in items {
            println!("{}", convertor.convert(&item));
        }
        return Ok(());
    }

    let mut input: Box<dyn BufRead> = if let Some(input) = args.input {
        Box::new(BufReader::new(File::open(input)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut output: Box<dyn Write> = if let Some(output) = args.output {
        Box::new(BufWriter::new(File::create(&output)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    output.write_all(convertor.convert(&buffer).as_bytes())?;

    Ok(())
}

impl Commands {
    fn run(self) -> io::Result<()> {
        match self {
            Commands::S2T(args) => convert(S2T.new(), args),
            Commands::S2TW(args) => convert(S2TW.new(), args),
            Commands::S2TWP(args) => convert(S2TWP.new(), args),
            Commands::T2S(args) => convert(T2S.new(), args),
            Commands::T2TW(args) => convert(T2TW.new(), args),
            Commands::TW2S(args) => convert(TW2S.new(), args),
            Commands::TW2SP(args) => convert(TW2SP.new(), args),
            Commands::TW2T(args) => convert(TW2T.new(), args),
            Commands::S2HK(args) => convert(S2HK.new(), args),
            Commands::HK2S(args) => convert(HK2S.new(), args),
            Commands::HK2T(args) => convert(HK2T.new(), args),
            Commands::T2HK(args) => convert(T2HK.new(), args),
            Commands::T2JP(args) => convert(T2JP.new(), args),
            Commands::JP2T(args) => convert(JP2T.new(), args),
        }
    }
}

#[derive(Args)]
struct Args_ {
    /// Input file
    #[arg(short, value_name = "PATH")]
    input: Option<String>,
    /// Output file
    #[arg(short, value_name = "PATH")]
    output: Option<String>,
    #[arg(conflicts_with_all = &["input", "output"])]
    items: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
    CLI::parse().command.run()
}
