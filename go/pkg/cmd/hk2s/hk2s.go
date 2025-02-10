package hk2s

import (
	"github.com/fhluo/hanconv/go/pkg/cc/hk2s"
	"github.com/fhluo/hanconv/go/pkg/cmd"
	"log/slog"
	"os"
)

var hk2sCmd = cmd.New(
	"hk2s",
	"繁体中文（香港） -> 简体中文",
	hk2s.Convert,
)

func Execute() {
	if err := hk2sCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
