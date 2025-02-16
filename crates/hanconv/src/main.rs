use clap::{Args, Parser, Subcommand};
use hanconv::{Convertor, Convertors::*};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

#[derive(Parser)]
#[command(version, about)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simplified Chinese to Traditional Chinese
    ///
    /// 简体中文 → 繁体中文
    S2T(ConvertArgs),
    /// Traditional Chinese to Simplified Chinese
    ///
    /// 繁体中文 → 简体中文
    T2S(ConvertArgs),
    /// Simplified Chinese to Traditional Chinese (Taiwan)
    ///
    /// 简体中文 → 繁体中文（台湾）
    S2TW(ConvertArgs),
    /// Traditional Chinese (Taiwan) to Simplified Chinese
    ///
    /// 繁体中文（台湾）→ 简体中文
    TW2S(ConvertArgs),
    /// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
    ///
    /// 简体中文 → 繁体中文（台湾），转换为台湾常用词
    S2TWP(ConvertArgs),
    /// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
    ///
    /// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
    TW2SP(ConvertArgs),
    /// Traditional Chinese to Traditional Chinese (Taiwan)
    ///
    /// 繁体中文 → 繁体中文（台湾）
    T2TW(ConvertArgs),
    /// Traditional Chinese (Taiwan) to Traditional Chinese
    ///
    /// 繁体中文（台湾）→ 繁体中文
    TW2T(ConvertArgs),
    /// Simplified Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 简体中文 → 繁体中文（香港）
    S2HK(ConvertArgs),
    /// Traditional Chinese (Hong Kong) to Simplified Chinese
    ///
    /// 繁体中文（香港）→ 简体中文
    HK2S(ConvertArgs),
    /// Traditional Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 繁体中文 → 繁体中文（香港）
    T2HK(ConvertArgs),
    /// Traditional Chinese (Hong Kong) to Traditional Chinese
    ///
    /// 繁体中文（香港）→ 繁体中文
    HK2T(ConvertArgs),
    /// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    ///
    /// 繁体字 → 日文新字体
    T2JP(ConvertArgs),
    /// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
    ///
    /// 日文新字体 → 繁体字
    JP2T(ConvertArgs),
}

fn convert(convertor: Convertor, args: ConvertArgs) -> io::Result<()> {
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
struct ConvertArgs {
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
