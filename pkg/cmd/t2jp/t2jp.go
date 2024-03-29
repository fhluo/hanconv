package t2jp

import (
	"github.com/fhluo/gocc/pkg/cc/t2jp"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var t2jpCmd = cmd.New(
    "t2jp",
    "繁体中文 -> 日文汉字（新字体）",
    t2jp.Convert,
)

func Execute() {
	if err := t2jpCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
