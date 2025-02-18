package s2hk

import (
	"github.com/fhluo/hanconv/go/internal/cmd"
	"github.com/fhluo/hanconv/go/pkg/s2hk"
	"log/slog"
	"os"
)

var s2hkCmd = cmd.New(
	"s2hk",
	"简体中文 -> 繁体中文（香港）",
	s2hk.Convert,
)

func Execute() {
	if err := s2hkCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
