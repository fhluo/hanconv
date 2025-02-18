package tw2t

import (
	"github.com/fhluo/hanconv/go/internal/cmd"
	"github.com/fhluo/hanconv/go/pkg/tw2t"
	"log/slog"
	"os"
)

var tw2tCmd = cmd.New(
	"tw2t",
	"繁体中文（台湾） -> 繁体中文",
	tw2t.Convert,
)

func Execute() {
	if err := tw2tCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
