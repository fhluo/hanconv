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

- **Specify input and output encoding:**

    ```shell
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    ```

### As a Library

1. Add `hanconv` as a dependency in your `Cargo.toml`.

    ```toml
    [dependencies]
    hanconv = "0.2"
    ```

2. Use the provided conversion functions in your project.

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
