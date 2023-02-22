package main

import (
	"github.com/bytedance/sonic"
	"github.com/fhluo/hanconv/pkg/hanconv"
	"github.com/fhluo/hanconv/pkg/trie"
	"github.com/samber/lo"
	"golang.org/x/exp/slog"
	"log"
	"os"
	"path/filepath"
)

func init() {
	log.SetFlags(0)
}

func main() {
	cc, err := NewOpenCC()
	if err != nil {
		slog.Error("", err)
		os.Exit(1)
	}

	//dictionaries, err := cc.Dictionaries()
	//if err != nil {
	//	slog.Error("无法获取字典文件名", err)
	//	os.Exit(1)
	//}

	configs, err := cc.Configs()
	if err != nil {
		slog.Error("无法获取配置文件名", err)
		os.Exit(1)
	}

	for _, filename := range configs {
		config, err := cc.ReadConfig(filename)
		if err != nil {
			slog.Error("无法读取配置", err, "config", filename)
		}

		data, err := sonic.Marshal(hanconv.New(
			lo.Map(config.ConversionChain, func(conversion Conversion, _ int) *trie.Trie {
				return trie.FromMap(
					lo.Map(conversion.Dictionary.Files(), func(path string, _ int) map[string]string {
						path = path[:len(path)-len(filepath.Ext(path))] + ".txt"
						dictionary, err := cc.ReadDictionary(path)
						if err != nil {
							slog.Error("无法读取字典", err)
							os.Exit(1)
						}
						return dictionary
					})...,
				)
			})...,
		))
		if err != nil {
			slog.Error("将 Converter 序列化为 JSON 失败", err)
			os.Exit(1)
		}

		err = os.WriteFile(filename[:len(filename)-len(filepath.Ext(filename))]+".json", data, 0666)
		if err != nil {
			slog.Error("", err)
			os.Exit(1)
		}
	}
}
