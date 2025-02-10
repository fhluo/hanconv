package jp2t

import (
	"github.com/fhluo/hanconv/go/pkg/cc/jp2t"
	"github.com/fhluo/hanconv/go/pkg/cmd"
	"log/slog"
	"os"
)

var jp2tCmd = cmd.New(
	"jp2t",
	"日文汉字（新字体） -> 繁体中文",
	jp2t.Convert,
)

func Execute() {
	if err := jp2tCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
