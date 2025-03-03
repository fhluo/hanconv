# Hanconv

Hanconv converts between Chinese characters variants base on [OpenCC](https://github.com/BYVoid/OpenCC)'s dictionaries
and conversion rules. It provides efficient conversion between Simplified Chinese and Traditional Chinese both as a
command-line tool and as a library for integration into other projects.

## Installation

```shell
cargo install hanconv
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

- **Convert text directly from command line:**

    ```shell
    hanconv s2t "简繁转换"
    ```

- **Read from stdin and write to stdout:**

    ```shell
    cat input.txt | hanconv s2t > output.txt
    ```

- **For files with non-UTF-8 encodings, specify the encoding:**

    ```shell
    # Same encoding for input and output
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    
    # Different encodings for input and output
    hanconv t2s -i input.txt -o output.txt --input-encoding GBK --output-encoding UTF-8
    ```

### As a Library

1. Add `hanconv` as a dependency in your `Cargo.toml`.

    ```toml
    [dependencies]
    hanconv = "0.3"
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
