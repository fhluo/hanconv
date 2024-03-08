package t2tw

import (
	"github.com/fhluo/gocc/pkg/cc/t2tw"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var t2twCmd = cmd.New(
    "t2tw",
    "繁体中文 -> 繁体中文(台湾)",
    t2tw.Convert,
)

func Execute() {
	if err := t2twCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
