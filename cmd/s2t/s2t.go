package main

import (
	"github.com/fhluo/zhconv/cmd"
	"github.com/fhluo/zhconv/conv"
	"github.com/fhluo/zhconv/dict/st"
)

func main() {
	STConv := conv.New()
	STConv.UpdateDict(st.Characters, st.Phrases)
	cmd.Execute(STConv, "s2t", "简体中文 -> 繁体中文")
}
