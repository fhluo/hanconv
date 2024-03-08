package s2tw

import (
	"github.com/fhluo/gocc/pkg/cc/s2tw"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var s2twCmd = cmd.New(
    "s2tw",
    "简体中文 -> 繁体中文（台湾）",
    s2tw.Convert,
)

func Execute() {
	if err := s2twCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
