<div align="center">

# Hanconv

汉字转换
<br><br>
<a href="https://crates.io/crates/hanconv">
<img src="https://shieldcn.dev/group/crates/hanconv+crates/d/hanconv+github/license/fhluo/hanconv.svg?variant=branded&size=xs&mode=light" alt="Hanconv 在 crates.io 上的版本与下载量"></a>

<samp>

**[English](README.md)** ┃ **[简体中文](README.zh-Hans.md)**

</samp>

</div>

Hanconv 是一款基于 [OpenCC](https://github.com/BYVoid/OpenCC) 词库和转换规则的汉字简繁转换工具。它提供了命令行工具，并支持作为库集成到 Rust、Go、Python 和 JavaScript 项目中，用于在不同汉字变体之间进行转换。

## 目录

- [命令行工具](#命令行工具)
- [Rust 库](#rust-库)
- [Go 库](#go-库)
- [Python 库](#python-库)
- [JavaScript 库](#javascript-库)
- [支持的转换](#支持的转换)

## 命令行工具

### 安装

使用 Cargo 从 crates.io 安装 `hanconv` 命令（Rust 1.88+）：

```shell
cargo install hanconv-cli
```

#### 其他安装方式

从 PyPI 使用 pip 安装（需要 Python 3.11+）：

```shell
pip install hanconv-cli
```

使用 [uv](https://docs.astral.sh/uv/)：

```shell
uv tool install hanconv-cli
```

从 Go 包安装（Go 1.27+）：

```shell
go install github.com/fhluo/hanconv/go/cmd/hanconv@latest
```

预编译二进制可从 GitHub [Releases](https://github.com/fhluo/hanconv/releases/latest) 下载。

### 常用示例

- 查看帮助信息：

    ```shell
    hanconv --help
    ```

- 文件转换（简体 → 繁体）：

    ```shell
    hanconv s2t -i input.txt -o output.txt
    ```

- 文件转换（繁体 → 简体）：

    ```shell
    hanconv t2s -i input.txt -o output.txt
    ```

- 直接转换文本字符串：

    ```shell
    hanconv s2t "简繁转换"
    ```

- 使用管道（标准输入/输出）：

    ```shell
    cat input.txt | hanconv s2t > output.txt
    ```

- 指定文件编码（支持非 UTF-8）：

    ```shell
    # 输入和输出使用相同编码
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    
    # 输入和输出使用不同编码
    hanconv t2s -i input.txt -o output.txt --input-encoding GBK --output-encoding UTF-8
    ```

## Rust 库

1. 添加依赖：

    在 `Cargo.toml` 中添加 `hanconv`：

    ```toml
    [dependencies]
    hanconv = "0.6"
    ```

2. 调用转换函数：

    ```rust
    fn main() {
        let result = hanconv::s2t("简繁转换");
        assert_eq!(result, "簡繁轉換");
    }
    ```

3. 支持的转换模式：
    - `s2t`: 简体 → 繁体
    - `t2s`: 繁体 → 简体
    - `s2tw`: 简体 → 繁体（台湾）
    - `tw2s`: 繁体（台湾）→ 简体
    - `s2twp`: 简体 → 繁体（台湾），转换为台湾常用词
    - `tw2sp`: 繁体（台湾）→ 简体，转换为中国大陆常用词
    - `t2tw`: 繁体 → 繁体（台湾）
    - `tw2t`: 繁体（台湾）→ 繁体
    - `s2hk`: 简体 → 繁体（香港）
    - `hk2s`: 繁体（香港）→ 简体
    - `t2hk`: 繁体 → 繁体（香港）
    - `hk2t`: 繁体（香港）→ 繁体
    - `t2jp`: 繁体字 → 日文新字体
    - `jp2t`: 日文新字体 → 繁体字

## Go 库

1. 获取模块：

    ```shell
    go get github.com/fhluo/hanconv/go
    ```

2. 导入并使用：

    ```go
    package main

    import (
        "fmt"

        "github.com/fhluo/hanconv/go"
    )

    func main() {
        result := hanconv.S2T("简繁转换")
        fmt.Println(result)
        // 输出: 簡繁轉換
    }
    ```

## Python 库

1. 安装 Python 绑定：

    ```shell
    pip install hanconv
    ```

    或者使用 uv：

    ```shell
    uv add hanconv
    ```

2. 导入并使用转换函数：

    ```python
    import hanconv

    result = hanconv.s2t("简繁转换")
    assert result == "簡繁轉換"
    ```

## JavaScript 库

1. 从 npm 安装：

    ```shell
    npm install hanconv
    ```

    或者使用 Bun：

    ```shell
    bun add hanconv
    ```

2. 先加载 WebAssembly 模块，再调用转换函数：

    ```js
    import init, {s2t} from "hanconv";

    await init();

    console.log(s2t("简繁转换"));
    // 输出: 簡繁轉換
    ```

    Node.js 不能请求 `file:` URL，需要改为传入文件内容：

    ```js
    import {readFile} from "node:fs/promises";

    import init, {s2t} from "hanconv";

    const wasm = new URL("./hanconv_bg.wasm", import.meta.resolve("hanconv"));
    await init({module_or_path: await readFile(wasm)});
    ```

3. 可用的转换函数：`s2t`、`t2s`、`s2tw`、`tw2s`、`s2twp`、`tw2sp`、`t2tw`、`tw2t`、`s2hk`、`hk2s`、`t2hk`、`hk2t`、
   `t2jp`、`jp2t`。

## 支持的转换

|  转换   |    源     |        目标         |
|:-----:|:--------:|:-----------------:|
|  S2T  |   简体中文   |       繁体中文        |
|  T2S  |   繁体中文   |       简体中文        |
| S2TW  |   简体中文   |     繁体中文（台湾）      |
| TW2S  | 繁体中文（台湾） |       简体中文        |
| S2TWP |   简体中文   | 繁体中文（台湾），转换为台湾常用词 |
| TW2SP | 繁体中文（台湾） |  简体中文，转换为中国大陆常用词  |
| T2TW  |   繁体中文   |     繁体中文（台湾）      |
| TW2T  | 繁体中文（台湾） |       繁体中文        |
| S2HK  |   简体中文   |     繁体中文（香港）      |
| HK2S  | 繁体中文（香港） |       简体中文        |
| T2HK  |   繁体中文   |     繁体中文（香港）      |
| HK2T  | 繁体中文（香港） |       繁体中文        |
| T2JP  |   繁体字    |       日文新字体       |
| JP2T  |  日文新字体   |        繁体字        |

## 许可

源代码以 [MIT 许可证](https://github.com/fhluo/hanconv/blob/main/LICENSE) 授权。

Hanconv 内置的词典文件源自 [OpenCC](https://github.com/BYVoid/OpenCC)，以 [Apache License 2.0](https://github.com/fhluo/hanconv/blob/main/hanconv/data/opencc/LICENSE) 分发。
