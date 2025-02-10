package main

import (
	_ "embed"
	"fmt"
	"log"
	"log/slog"
	"os"
	"slices"
)

func init() {
	log.SetFlags(0)
}

func main() {
	openCC, err := NewOpenCC()
	if err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}

	configs, err := openCC.Configs()
	if err != nil {
		slog.Error("无法获取配置文件名", "err", err)
		os.Exit(1)
	}

	fmt.Println(slices.Collect(configs))
}
