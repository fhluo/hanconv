package t2hk

import (
	"github.com/fhluo/gocc/pkg/cc/t2hk"
	"github.com/fhluo/gocc/pkg/cmd"
    "log/slog"
    "os"
)

var t2hkCmd = cmd.New(
    "t2hk",
    "繁体中文 -> 繁体中文(香港)",
    t2hk.Convert,
)

func Execute() {
	if err := t2hkCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
