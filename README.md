<div align="center">

# Hanconv

Convert between Chinese character variants.
<br><br>
<a href="https://crates.io/crates/hanconv">
<img src="https://shieldcn.dev/group/crates/hanconv+crates/d/hanconv+github/license/fhluo/hanconv.svg?variant=branded&size=xs&mode=light" alt="Hanconv crate version and downloads on crates.io"></a>

<samp>

**[English](README.md)** ┃ **[简体中文](README.zh-Hans.md)**

</samp>

</div>

Hanconv converts between Chinese character variants based on [OpenCC](https://github.com/BYVoid/OpenCC)'s dictionaries
and conversion rules. It provides efficient conversion between Simplified Chinese and Traditional Chinese both as a
command-line tool and as a library for integration into Rust, Go, Python, and JavaScript projects.

## Table of Contents

- [Command-Line Tool](#command-line-tool)
- [Rust Library](#rust-library)
- [Go Library](#go-library)
- [Python Library](#python-library)
- [JavaScript Library](#javascript-library)
- [Supported Conversions](#supported-conversions)

## Command-Line Tool

### Installation

Install the `hanconv` command from crates.io with Cargo (Rust 1.88+):

```shell
cargo install hanconv-cli
```

#### Other Installation Methods

From PyPI with pip (Python 3.11+):

```shell
pip install hanconv-cli
```

Using [uv](https://docs.astral.sh/uv/):

```shell
uv tool install hanconv-cli
```

From the Go module (Go 1.27+):

```shell
go install github.com/fhluo/hanconv/go/cmd/hanconv@latest
```

Prebuilt binaries are available from GitHub
[Releases](https://github.com/fhluo/hanconv/releases/latest).

### Basic Usage

- Display the help message:

    ```shell
    hanconv --help
    ```

- Convert a text file from Simplified to Traditional Chinese:

    ```shell
    hanconv s2t -i input.txt -o output.txt
    ```

- Convert a text file from Traditional to Simplified Chinese:

    ```shell
    hanconv t2s -i input.txt -o output.txt
    ```

- Convert text directly from the command line:

    ```shell
    hanconv s2t "简繁转换"
    ```

- Read from stdin and write to stdout:

    ```shell
    cat input.txt | hanconv s2t > output.txt
    ```

- For files with non-UTF-8 encodings, specify the encoding:

    ```shell
    # Same encoding for input and output
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    
    # Different encodings for input and output
    hanconv t2s -i input.txt -o output.txt --input-encoding GBK --output-encoding UTF-8
    ```

## Rust Library

1. Add `hanconv` as a dependency in your `Cargo.toml`.

    ```toml
    [dependencies]
    hanconv = "0.6"
    ```

2. Use the provided conversion functions in your project.

    ```rust
    fn main() {
        let result = hanconv::s2t("简繁转换");
        assert_eq!(result, "簡繁轉換");
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

## Go Library

1. Add hanconv to your Go project:

    ```shell
    go get github.com/fhluo/hanconv/go
    ```

2. Import and use the conversion functions in your project:

    ```go
    package main

    import (
        "fmt"

        "github.com/fhluo/hanconv/go"
    )

    func main() {
        result := hanconv.S2T("简繁转换")
        fmt.Println(result)
        // Output: 簡繁轉換
    }
    ```

## Python Library

1. Install the package with pip:

    ```shell
    pip install hanconv
    ```

    Or with uv:

    ```shell
    uv add hanconv
    ```

2. Import and use the conversion functions in your project:

    ```python
    import hanconv

    result = hanconv.s2t("简繁转换")
    assert result == "簡繁轉換"
    ```

## JavaScript Library

1. Install the package from npm:

    ```shell
    npm install hanconv
    ```

    Or with Bun:

    ```shell
    bun add hanconv
    ```

2. Load the WebAssembly module once, then call the conversion functions:

    ```js
    import init, {s2t} from "hanconv";

    await init();

    console.log(s2t("简繁转换"));
    // Output: 簡繁轉換
    ```

    Node.js cannot fetch `file:` URLs, so read the module bytes and pass them:

    ```js
    import {readFile} from "node:fs/promises";

    import init, {s2t} from "hanconv";

    const wasm = new URL("./hanconv_bg.wasm", import.meta.resolve("hanconv"));
    await init({module_or_path: await readFile(wasm)});
    ```

3. Available conversion functions: `s2t`, `t2s`, `s2tw`, `tw2s`, `s2twp`, `tw2sp`, `t2tw`, `tw2t`, `s2hk`, `hk2s`,
   `t2hk`, `hk2t`, `t2jp`, `jp2t`.

## Supported Conversions

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

## License

The source code is licensed under the
[MIT License](https://github.com/fhluo/hanconv/blob/main/LICENSE).

The dictionary files bundled with Hanconv are derived from
[OpenCC](https://github.com/BYVoid/OpenCC) and are distributed under the
[Apache License 2.0](https://github.com/fhluo/hanconv/blob/main/hanconv/data/opencc/LICENSE).
