package main

import (
	"github.com/fhluo/hanzi-conv/cmd"
	"github.com/fhluo/hanzi-conv/pkg/hanzi/conv"
	st2 "github.com/fhluo/hanzi-conv/pkg/hanzi/dict/st"
)

func main() {
	STConv := conv.New()
	STConv.UpdateDict(st2.Characters, st2.Phrases)
	cmd.Execute(STConv, "s2t", "简体中文 -> 繁体中文")
}
