package tw2s

import (
	"github.com/fhluo/gocc/pkg/cc/tw2s"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var tw2sCmd = cmd.New(
    "tw2s",
    "繁体中文（台湾） -> 简体中文",
    tw2s.Convert,
)

func Execute() {
	if err := tw2sCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
