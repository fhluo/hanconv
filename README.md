<div align="center">

# Hanconv

Convert between Chinese characters variants.

<samp>

**[English](README.md)** ┃ **[简体中文](README.zh-Hans.md)**

</samp>

</div>

## Introduction

Hanconv converts between Chinese characters variants base on [OpenCC](https://github.com/BYVoid/OpenCC)'s dictionaries
and conversion rules. It provides efficient conversion between Simplified Chinese and Traditional Chinese both as a
command-line tool and as a library for integration into other projects.

## Installation

### Using Cargo

```shell
cargo install hanconv
```

### Using Go

```shell
go install github.com/fhluo/hanconv/go/cmd/hanconv@latest
```

## Usage

### Command-Line Interface

- **Display the help message:**

    ```shell
    hanconv --help
    ```

- **Convert a text file from Simplified to Traditional Chinese:**

    ```shell
    hanconv s2t -i input.txt -o output.txt
    ```

- **Convert a text file from Traditional to Simplified Chinese:**

    ```shell
    hanconv t2s -i input.txt -o output.txt
    ```

- **Specify input and output encoding:**

    ```shell
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    ```

### Rust

1. Add `hanconv` as a dependency in your `Cargo.toml`.

    ```toml
    [dependencies]
    hanconv = "0.2"
    ```

2. Use the provided conversion functions in your project.

    ```rust
    fn main() {
        let result = hanconv::s2t("简繁转换");
        println!("{}", result);
    }
    ```

3. Available conversion functions:
    - `s2t`: Simplified Chinese → Traditional Chinese
    - `t2s`: Traditional Chinese → Simplified Chinese
    - `s2tw`: Simplified Chinese → Traditional Chinese (Taiwan)
    - `tw2s`: Traditional Chinese (Taiwan) → Simplified Chinese
    - `s2twp`: Simplified Chinese → Traditional Chinese (Taiwan) with Taiwanese idiom
    - `tw2sp`: Traditional Chinese (Taiwan) → Simplified Chinese with Mainland Chinese idiom
    - `t2tw`: Traditional Chinese → Traditional Chinese (Taiwan)
    - `tw2t`: Traditional Chinese (Taiwan) → Traditional Chinese
    - `s2hk`: Simplified Chinese → Traditional Chinese (Hong Kong)
    - `hk2s`: Traditional Chinese (Hong Kong) → Simplified Chinese
    - `t2hk`: Traditional Chinese → Traditional Chinese (Hong Kong)
    - `hk2t`: Traditional Chinese (Hong Kong) → Traditional Chinese
    - `t2jp`: Traditional Chinese characters (Kyūjitai) → New Japanese Kanji (Shinjitai)
    - `jp2t`: New Japanese Kanji (Shinjitai) → Traditional Chinese characters (Kyūjitai)

### Go

1. Add hanconv to your Go project:

    ```shell
    go get github.com/fhluo/hanconv/go
    ```

2. Import and use the conversion functions in your project:

    ```go
    import hanconv "github.com/fhluo/hanconv/go"

    func main() {
        result := hanconv.S2T("简繁转换")
        fmt.Println(result)
    }
    ```

## Conversions

| Conversion |                  Source                   |                      Target                       |
|:----------:|:-----------------------------------------:|:-------------------------------------------------:|
|    S2T     |            Simplified Chinese             |                Traditional Chinese                |
|    T2S     |            Traditional Chinese            |                Simplified Chinese                 |
|    S2TW    |            Simplified Chinese             |           Traditional Chinese (Taiwan)            |
|    TW2S    |       Traditional Chinese (Taiwan)        |                Simplified Chinese                 |
|   S2TWP    |            Simplified Chinese             | Traditional Chinese (Taiwan) with Taiwanese idiom |
|   TW2SP    |       Traditional Chinese (Taiwan)        |  Simplified Chinese with Mainland Chinese idiom   |
|    T2TW    |            Traditional Chinese            |           Traditional Chinese (Taiwan)            |
|    TW2T    |       Traditional Chinese (Taiwan)        |                Traditional Chinese                |
|    S2HK    |            Simplified Chinese             |          Traditional Chinese (Hong Kong)          |
|    HK2S    |      Traditional Chinese (Hong Kong)      |                Simplified Chinese                 |
|    T2HK    |            Traditional Chinese            |          Traditional Chinese (Hong Kong)          |
|    HK2T    |      Traditional Chinese (Hong Kong)      |                Traditional Chinese                |
|    T2JP    | Traditional Chinese characters (Kyūjitai) |          New Japanese Kanji (Shinjitai)           |
|    JP2T    |      New Japanese Kanji (Shinjitai)       |     Traditional Chinese characters (Kyūjitai)     |
