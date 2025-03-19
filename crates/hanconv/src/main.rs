use clap::{Args, Parser, Subcommand};
use encoding_rs::{Encoding, UTF_8};
use hanconv::{Converter, Converters::*};
use rayon::prelude::*;
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

impl CLI {
    fn run(self) -> Result<(), Box<dyn Error>> {
        self.command.run()
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Convert Simplified Chinese to Traditional Chinese
    ///
    /// 简体中文 → 繁体中文
    S2T(Conversion),
    /// Convert Traditional Chinese to Simplified Chinese
    ///
    /// 繁体中文 → 简体中文
    T2S(Conversion),
    /// Convert Simplified Chinese to Traditional Chinese (Taiwan)
    ///
    /// 简体中文 → 繁体中文（台湾）
    S2TW(Conversion),
    /// Convert Traditional Chinese (Taiwan) to Simplified Chinese
    ///
    /// 繁体中文（台湾）→ 简体中文
    TW2S(Conversion),
    /// Convert Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
    ///
    /// 简体中文 → 繁体中文（台湾），转换为台湾常用词
    S2TWP(Conversion),
    /// Convert Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
    ///
    /// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
    TW2SP(Conversion),
    /// Convert Traditional Chinese to Traditional Chinese (Taiwan)
    ///
    /// 繁体中文 → 繁体中文（台湾）
    T2TW(Conversion),
    /// Convert Traditional Chinese (Taiwan) to Traditional Chinese
    ///
    /// 繁体中文（台湾）→ 繁体中文
    TW2T(Conversion),
    /// Convert Simplified Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 简体中文 → 繁体中文（香港）
    S2HK(Conversion),
    /// Convert Traditional Chinese (Hong Kong) to Simplified Chinese
    ///
    /// 繁体中文（香港）→ 简体中文
    HK2S(Conversion),
    /// Convert Traditional Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 繁体中文 → 繁体中文（香港）
    T2HK(Conversion),
    /// Convert Traditional Chinese (Hong Kong) to Traditional Chinese
    ///
    /// 繁体中文（香港）→ 繁体中文
    HK2T(Conversion),
    /// Convert Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    ///
    /// 繁体字 → 日文新字体
    T2JP(Conversion),
    /// Convert New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
    ///
    /// 日文新字体 → 繁体字
    JP2T(Conversion),
}

impl Commands {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::S2T(conversion) => conversion.run(S2T.new_converter()),
            Commands::T2S(conversion) => conversion.run(T2S.new_converter()),
            Commands::S2TW(conversion) => conversion.run(S2TW.new_converter()),
            Commands::TW2S(conversion) => conversion.run(TW2S.new_converter()),
            Commands::S2TWP(conversion) => conversion.run(S2TWP.new_converter()),
            Commands::TW2SP(conversion) => conversion.run(TW2SP.new_converter()),
            Commands::T2TW(conversion) => conversion.run(T2TW.new_converter()),
            Commands::TW2T(conversion) => conversion.run(TW2T.new_converter()),
            Commands::S2HK(conversion) => conversion.run(S2HK.new_converter()),
            Commands::HK2S(conversion) => conversion.run(HK2S.new_converter()),
            Commands::T2HK(conversion) => conversion.run(T2HK.new_converter()),
            Commands::HK2T(conversion) => conversion.run(HK2T.new_converter()),
            Commands::T2JP(conversion) => conversion.run(T2JP.new_converter()),
            Commands::JP2T(conversion) => conversion.run(JP2T.new_converter()),
        }
    }
}

#[derive(Args)]
struct Conversion {
    #[arg(skip)]
    converter: Option<Converter>,
    /// Input file path (stdin if not specified)
    #[arg(short, value_name = "PATH")]
    input_filename: Option<String>,
    /// Output file path (stdout if not specified)
    #[arg(short, value_name = "PATH")]
    output_filename: Option<String>,
    /// Auto-generate output filename by converting input filename
    ///
    /// Adds suffix if names are identical
    #[arg(short, requires = "input_filename", conflicts_with = "output_filename")]
    generate_output_filename: bool,
    /// Suffix for auto-generated filenames
    #[arg(
        long,
        requires = "generate_output_filename",
        conflicts_with = "output_filename",
        default_value = "_converted"
    )]
    suffix: String,
    /// Set both input and output encoding
    #[arg(long, conflicts_with_all = &["input_encoding", "output_encoding"])]
    encoding: Option<String>,
    /// Set input encoding
    ///
    /// [default: UTF-8]
    #[arg(long, value_name = "ENCODING")]
    input_encoding: Option<String>,
    /// Set output encoding
    ///
    /// [default: UTF-8]
    #[arg(long, value_name = "ENCODING")]
    output_encoding: Option<String>,
    /// Text to convert directly from command line
    #[arg(value_name = "TEXT", exclusive = true)]
    texts: Option<Vec<String>>,
}

impl Conversion {
    fn handle_texts(self) -> Result<(), Box<dyn Error>> {
        let converter = self.converter.unwrap_or_else(|| T2S.new_converter());

        if let Some(ref texts) = self.texts {
            let mut writer = BufWriter::new(io::stdout());
            for text in texts {
                write!(writer, "{}", converter.convert(text))?;
            }
            writer.flush()?;
        }

        Ok(())
    }

    fn input_encoding(&self) -> &'static Encoding {
        self.input_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
            .unwrap_or(UTF_8)
    }

    fn output_encoding(&self) -> &'static Encoding {
        self.output_encoding
            .as_ref()
            .or(self.encoding.as_ref())
            .and_then(|encoding| Encoding::for_label(encoding.as_bytes()))
            .unwrap_or(UTF_8)
    }

    fn input(&self) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        let input: Box<dyn BufRead> = if let Some(ref filename) = self.input_filename {
            Box::new(BufReader::new(File::open(filename)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };

        Ok(input)
    }

    fn output(&self) -> Result<Box<dyn Write>, Box<dyn Error>> {
        let output: Box<dyn Write> = if let Some(ref filename) = self.output_filename {
            Box::new(BufWriter::new(File::create(filename)?))
        } else if let Some(filename) = self.generate_output_filename() {
            Box::new(BufWriter::new(File::create(filename)?))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        Ok(output)
    }

    fn generate_output_filename(&self) -> Option<impl AsRef<Path>> {
        if self.input_filename.is_none() || !self.generate_output_filename {
            return None;
        }

        let input_filename = self.input_filename.as_ref().unwrap();
        let output_filename = &self.converter.as_ref().unwrap().convert(input_filename);

        if input_filename != output_filename {
            return Some(output_filename.into());
        }

        let mut path = PathBuf::from(output_filename);
        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let ext = path.extension().unwrap_or_default().to_string_lossy();

        path.set_file_name(stem + self.suffix.as_str() + &ext);

        Some(path)
    }

    fn use_default_encoding(&self) -> bool {
        matches!(
            self,
            Conversion {
                encoding: None,
                input_encoding: None,
                output_encoding: None,
                ..
            }
        )
    }

    fn decode<'a>(&self, buffer: &'a [u8]) -> Result<Cow<'a, str>, Box<dyn Error>> {
        let encoding = self.input_encoding();
        let (cow, _, err) = encoding.decode(buffer);
        if err {
            Err(format!("Error decoding in encoding {}", encoding.name()).into())
        } else {
            Ok(cow)
        }
    }

    fn encode<'a>(&self, s: &'a str) -> Result<Cow<'a, [u8]>, Box<dyn Error>> {
        let encoding = self.output_encoding();
        let (cow, _, err) = encoding.encode(s);
        if err {
            Err(format!("Error encoding in encoding {}", encoding.name()).into())
        } else {
            Ok(cow)
        }
    }

    fn handle_io(self) -> Result<(), Box<dyn Error>> {
        let mut input = self.input()?;
        let mut output = self.output()?;

        let mut buffer = Vec::new();
        input.read_to_end(&mut buffer)?;

        let converter = self.converter.as_ref().unwrap();

        let s = if self.use_default_encoding() {
            Cow::Owned(String::from_utf8(buffer)?)
        } else {
            self.decode(&buffer)?
        };

        let s = s
            .par_split_inclusive('\n')
            .map(|s| converter.convert(s))
            .collect::<String>();

        if self.use_default_encoding() {
            output.write_all(s.as_bytes())?;
        } else {
            output.write_all(&self.encode(&s)?)?;
        }

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
    CLI::parse().run()
}
