package hk2t

import (
	"github.com/fhluo/hanconv/go/pkg/cmd"
	"github.com/fhluo/hanconv/go/pkg/hk2t"
	"log/slog"
	"os"
)

var hk2tCmd = cmd.New(
	"hk2t",
	"繁体中文（香港） -> 繁体中文",
	hk2t.Convert,
)

func Execute() {
	if err := hk2tCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
