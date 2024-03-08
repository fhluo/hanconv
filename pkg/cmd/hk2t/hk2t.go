package hk2t

import (
	"github.com/fhluo/gocc/pkg/cc/hk2t"
	"github.com/fhluo/gocc/pkg/cmd"
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
