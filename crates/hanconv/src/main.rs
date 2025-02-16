use clap::{Args, Parser, Subcommand};
use encoding_rs::{Encoding, UTF_8};
use hanconv::{Convertor, Convertors::*};
use std::error::Error;
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
    S2T(Command),
    /// Traditional Chinese to Simplified Chinese
    ///
    /// 繁体中文 → 简体中文
    T2S(Command),
    /// Simplified Chinese to Traditional Chinese (Taiwan)
    ///
    /// 简体中文 → 繁体中文（台湾）
    S2TW(Command),
    /// Traditional Chinese (Taiwan) to Simplified Chinese
    ///
    /// 繁体中文（台湾）→ 简体中文
    TW2S(Command),
    /// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
    ///
    /// 简体中文 → 繁体中文（台湾），转换为台湾常用词
    S2TWP(Command),
    /// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
    ///
    /// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
    TW2SP(Command),
    /// Traditional Chinese to Traditional Chinese (Taiwan)
    ///
    /// 繁体中文 → 繁体中文（台湾）
    T2TW(Command),
    /// Traditional Chinese (Taiwan) to Traditional Chinese
    ///
    /// 繁体中文（台湾）→ 繁体中文
    TW2T(Command),
    /// Simplified Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 简体中文 → 繁体中文（香港）
    S2HK(Command),
    /// Traditional Chinese (Hong Kong) to Simplified Chinese
    ///
    /// 繁体中文（香港）→ 简体中文
    HK2S(Command),
    /// Traditional Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 繁体中文 → 繁体中文（香港）
    T2HK(Command),
    /// Traditional Chinese (Hong Kong) to Traditional Chinese
    ///
    /// 繁体中文（香港）→ 繁体中文
    HK2T(Command),
    /// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    ///
    /// 繁体字 → 日文新字体
    T2JP(Command),
    /// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
    ///
    /// 日文新字体 → 繁体字
    JP2T(Command),
}

impl Commands {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::S2T(command) => command.run(S2T.new()),
            Commands::T2S(command) => command.run(T2S.new()),
            Commands::S2TW(command) => command.run(S2TW.new()),
            Commands::TW2S(command) => command.run(TW2S.new()),
            Commands::S2TWP(command) => command.run(S2TWP.new()),
            Commands::TW2SP(command) => command.run(TW2SP.new()),
            Commands::T2TW(command) => command.run(T2TW.new()),
            Commands::TW2T(command) => command.run(TW2T.new()),
            Commands::S2HK(command) => command.run(S2HK.new()),
            Commands::HK2S(command) => command.run(HK2S.new()),
            Commands::T2HK(command) => command.run(T2HK.new()),
            Commands::HK2T(command) => command.run(HK2T.new()),
            Commands::T2JP(command) => command.run(T2JP.new()),
            Commands::JP2T(command) => command.run(JP2T.new()),
        }
    }
}

#[derive(Args)]
struct Command {
    /// Input file
    #[arg(short, value_name = "PATH")]
    input: Option<String>,
    /// Output file
    #[arg(short, value_name = "PATH")]
    output: Option<String>,
    /// Specify input and output encoding
    #[arg(long, conflicts_with_all = &["input_encoding", "output_encoding"])]
    encoding: Option<String>,
    /// Specify input encoding
    #[arg(long, value_name = "ENCODING")]
    input_encoding: Option<String>,
    /// Specify output encoding
    #[arg(long, value_name = "ENCODING")]
    output_encoding: Option<String>,
    #[arg(conflicts_with_all = &["input", "output"])]
    items: Option<Vec<String>>,
}

impl Command {
    fn get_input_encoding(&self) -> Option<&'static Encoding> {
        self.input_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
    }

    fn get_output_encoding(&self) -> Option<&'static Encoding> {
        self.output_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
    }

    fn convert_items(&self, convertor: &Convertor) {
        if let Some(ref items) = self.items {
            for item in items {
                println!("{}", convertor.convert(item));
            }
        }
    }

    fn run(&self, convertor: Convertor) -> Result<(), Box<dyn Error>> {
        self.convert_items(&convertor);

        let mut input: Box<dyn BufRead> = if let Some(ref input) = self.input {
            Box::new(BufReader::new(File::open(input)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };

        let mut output: Box<dyn Write> = if let Some(ref output) = self.output {
            Box::new(BufWriter::new(File::create(&output)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;

        if matches!(
            self,
            Command {
                encoding: None,
                input_encoding: None,
                output_encoding: None,
                ..
            }
        ) {
            output.write_all(convertor.convert(String::from_utf8(buffer)?).as_bytes())?;
            return Ok(());
        }

        let input_encoding = self.get_input_encoding().unwrap_or(UTF_8);
        let output_encoding = self.get_output_encoding().unwrap_or(UTF_8);

        let (cow, _, err) = input_encoding.decode(&buffer);
        if err {
            return Err(format!("Error decoding in encoding {}", input_encoding.name()).into());
        }

        let r = convertor.convert(cow);
        let (cow, _, err) = output_encoding.encode(&r);
        if err {
            return Err(format!("Error encoding in encoding {}", output_encoding.name()).into());
        }
        output.write_all(cow.as_ref())?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    CLI::parse().command.run()
}
