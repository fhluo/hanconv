package tw2sp

import (
	"github.com/fhluo/hanconv/go/pkg/cmd"
	"github.com/fhluo/hanconv/go/pkg/tw2sp"
	"log/slog"
	"os"
)

var tw2spCmd = cmd.New(
	"tw2sp",
	"繁体中文（台湾） -> 简体中文（转换常用词汇）",
	tw2sp.Convert,
)

func Execute() {
	if err := tw2spCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
