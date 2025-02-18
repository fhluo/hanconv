package s2t

import (
	"github.com/fhluo/hanconv/go/internal/cmd"
	"github.com/fhluo/hanconv/go/pkg/s2t"
	"log/slog"
	"os"
)

var s2tCmd = cmd.New(
	"s2t",
	"简体中文 -> 繁体中文",
	s2t.Convert,
)

func Execute() {
	if err := s2tCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
