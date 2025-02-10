package s2twp

import (
	"github.com/fhluo/hanconv/go/pkg/cc/s2twp"
	"github.com/fhluo/hanconv/go/pkg/cmd"
	"log/slog"
	"os"
)

var s2twpCmd = cmd.New(
	"s2twp",
	"简体中文 -> 繁体中文（台湾）（转换常用词汇）",
	s2twp.Convert,
)

func Execute() {
	if err := s2twpCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
