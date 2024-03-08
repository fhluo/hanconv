package t2s

import (
	"github.com/fhluo/gocc/pkg/cc/t2s"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var t2sCmd = cmd.New(
    "t2s",
    "繁体中文 -> 简体中文",
    t2s.Convert,
)

func Execute() {
	if err := t2sCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
