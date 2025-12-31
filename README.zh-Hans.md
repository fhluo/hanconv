<div align="center">

# Hanconv

汉字转换
<br><br>
<a href="https://github.com/fhluo/hanconv/actions/workflows/build.yaml">
<img src="https://github.com/fhluo/hanconv/actions/workflows/build.yaml/badge.svg" alt="build workflow"></a>
<a href="https://crates.io/crates/hanconv">
<img src="https://img.shields.io/crates/v/hanconv" alt="version"></a>
<a href="https://pkg.go.dev/github.com/fhluo/hanconv/go">
<img src="https://img.shields.io/github/v/tag/fhluo/hanconv?filter=go%2F*&label=pkg"></a>

<samp>

**[English](README.md)** ┃ **[简体中文](README.zh-Hans.md)**

</samp>

</div>

## 介绍

Hanconv 是一款基于 [OpenCC](https://github.com/BYVoid/OpenCC) 词库和转换规则的汉字简繁转换工具。它提供了命令行工具，并支持作为库集成到 Rust 和 Go 项目中，用于在不同汉字变体之间进行转换。

## 安装

### 使用 Cargo

```shell
cargo install hanconv-cli
```

### 使用 Go

```shell
go install github.com/fhluo/hanconv/go/cmd/hanconv@latest
```

## 用法

### 命令行界面

- **查看帮助信息：**

    ```shell
    hanconv --help
    ```

- **文件转换（简体 → 繁体）：**

    ```shell
    hanconv s2t -i input.txt -o output.txt
    ```

- **文件转换（繁体 → 简体）：**

    ```shell
    hanconv t2s -i input.txt -o output.txt
    ```

- **直接转换文本字符串：**

    ```shell
    hanconv s2t "简繁转换"
    ```

- **使用管道（标准输入/输出）：**

    ```shell
    cat input.txt | hanconv s2t > output.txt
    ```

- **指定文件编码（支持非 UTF-8）：**

    ```shell
    # 输入和输出使用相同编码
    hanconv t2s -i input.txt -o output.txt --encoding GBK
    
    # 输入和输出使用不同编码
    hanconv t2s -i input.txt -o output.txt --input-encoding GBK --output-encoding UTF-8
    ```

### Rust

1. **添加依赖**：

    在 `Cargo.toml` 中添加 `hanconv`：

    ```toml
    [dependencies]
    hanconv = "0.5"
    ```

2. **调用转换函数**：

    ```rust
    fn main() {
        let result = hanconv::s2t("简繁转换");
        println!("{}", result);
    }
    ```

3. **支持的转换模式**：
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

### Go

1. **获取模块**：

    ```shell
    go get github.com/fhluo/hanconv/go
    ```

2. **导入并使用**：

    ```go
    import hanconv "github.com/fhluo/hanconv/go"

    func main() {
        result := hanconv.S2T("简繁转换")
        fmt.Println(result)
    }
    ```

## 转换类型

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
