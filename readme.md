# zhconv

简体中文与繁体中文文本转换工具

简繁对应表来自 [OpenCC](https://github.com/BYVoid/OpenCC)

## 安装

```shell
go install github.com/fhluo/zhconv/cmd/t2s@latest
go install github.com/fhluo/zhconv/cmd/s2t@latest
```

## CLI

### t2s

繁体中文 -> 简体中文

#### --input, -i

指定输入文件名，若未指定将使用标准输入

#### --output, -o

指定输出文件名， 若未指定将使用标准输出

### s2t

简体中文 -> 繁体中文

#### --input, -i

指定输入文件名，若未指定将使用标准输入

#### --output, -o

指定输出文件名， 若未指定将使用标准输出
