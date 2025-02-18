package t2hk

import (
	"github.com/fhluo/hanconv/go/internal/cmd"
	"github.com/fhluo/hanconv/go/pkg/t2hk"
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
