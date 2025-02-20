use clap::{Args, Parser, Subcommand};
use encoding_rs::{Encoding, UTF_8};
use hanconv::{Converter, Converters::*};
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
    S2T(Conversion),
    /// Traditional Chinese to Simplified Chinese
    ///
    /// 繁体中文 → 简体中文
    T2S(Conversion),
    /// Simplified Chinese to Traditional Chinese (Taiwan)
    ///
    /// 简体中文 → 繁体中文（台湾）
    S2TW(Conversion),
    /// Traditional Chinese (Taiwan) to Simplified Chinese
    ///
    /// 繁体中文（台湾）→ 简体中文
    TW2S(Conversion),
    /// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
    ///
    /// 简体中文 → 繁体中文（台湾），转换为台湾常用词
    S2TWP(Conversion),
    /// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
    ///
    /// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
    TW2SP(Conversion),
    /// Traditional Chinese to Traditional Chinese (Taiwan)
    ///
    /// 繁体中文 → 繁体中文（台湾）
    T2TW(Conversion),
    /// Traditional Chinese (Taiwan) to Traditional Chinese
    ///
    /// 繁体中文（台湾）→ 繁体中文
    TW2T(Conversion),
    /// Simplified Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 简体中文 → 繁体中文（香港）
    S2HK(Conversion),
    /// Traditional Chinese (Hong Kong) to Simplified Chinese
    ///
    /// 繁体中文（香港）→ 简体中文
    HK2S(Conversion),
    /// Traditional Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 繁体中文 → 繁体中文（香港）
    T2HK(Conversion),
    /// Traditional Chinese (Hong Kong) to Traditional Chinese
    ///
    /// 繁体中文（香港）→ 繁体中文
    HK2T(Conversion),
    /// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    ///
    /// 繁体字 → 日文新字体
    T2JP(Conversion),
    /// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
    ///
    /// 日文新字体 → 繁体字
    JP2T(Conversion),
}

impl Commands {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::S2T(conversion) => conversion.run(S2T.new()),
            Commands::T2S(conversion) => conversion.run(T2S.new()),
            Commands::S2TW(conversion) => conversion.run(S2TW.new()),
            Commands::TW2S(conversion) => conversion.run(TW2S.new()),
            Commands::S2TWP(conversion) => conversion.run(S2TWP.new()),
            Commands::TW2SP(conversion) => conversion.run(TW2SP.new()),
            Commands::T2TW(conversion) => conversion.run(T2TW.new()),
            Commands::TW2T(conversion) => conversion.run(TW2T.new()),
            Commands::S2HK(conversion) => conversion.run(S2HK.new()),
            Commands::HK2S(conversion) => conversion.run(HK2S.new()),
            Commands::T2HK(conversion) => conversion.run(T2HK.new()),
            Commands::HK2T(conversion) => conversion.run(HK2T.new()),
            Commands::T2JP(conversion) => conversion.run(T2JP.new()),
            Commands::JP2T(conversion) => conversion.run(JP2T.new()),
        }
    }
}

#[derive(Args)]
struct Conversion {
    #[arg(skip)]
    converter: Option<Converter>,
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
    texts: Option<Vec<String>>,
}

impl Conversion {
    fn handle_texts(self) -> Result<(), Box<dyn Error>> {
        let converter = self.converter.unwrap_or_else(|| T2S.new());

        if let Some(ref texts) = self.texts {
            let mut writer = BufWriter::new(io::stdout());
            for text in texts {
                write!(writer, "{}", converter.convert(text))?;
            }
            writer.flush()?;
        }

        Ok(())
    }

    fn handle_io(self) -> Result<(), Box<dyn Error>> {
        let converter = self.converter.unwrap_or_else(|| T2S.new());

        let mut input: Box<dyn BufRead> = if let Some(input) = self.input {
            Box::new(BufReader::new(File::open(input)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };

        let mut output: Box<dyn Write> = if let Some(output) = self.output {
            Box::new(BufWriter::new(File::create(output)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;

        if matches!(
            self,
            Conversion {
                encoding: None,
                input_encoding: None,
                output_encoding: None,
                ..
            }
        ) {
            output.write_all(converter.convert(String::from_utf8(buffer)?).as_bytes())?;
            return Ok(());
        }

        let input_encoding = self
            .input_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
            .unwrap_or(UTF_8);
        let output_encoding = self
            .output_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
            .unwrap_or(UTF_8);

        let (cow, _, err) = input_encoding.decode(&buffer);
        if err {
            return Err(format!("Error decoding in encoding {}", input_encoding.name()).into());
        }

        let r = converter.convert(cow);
        let (cow, _, err) = output_encoding.encode(&r);
        if err {
            return Err(format!("Error encoding in encoding {}", output_encoding.name()).into());
        }
        output.write_all(cow.as_ref())?;

        Ok(())
    }

    fn run(mut self, converter: Converter) -> Result<(), Box<dyn Error>> {
        self.converter = Some(converter);

        if self.texts.is_some() {
            self.handle_texts()?;
        } else {
            self.handle_io()?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    CLI::parse().command.run()
}
