package main

import (
	"github.com/fhluo/hanzi-conv/cmd"
	"github.com/fhluo/hanzi-conv/conv"
	"github.com/fhluo/hanzi-conv/dict/st"
)

func main() {
	STConv := conv.New()
	STConv.UpdateDict(st.Characters, st.Phrases)
	cmd.Execute(STConv, "s2t", "简体中文 -> 繁体中文")
}
