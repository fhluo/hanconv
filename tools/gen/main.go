package main

import (
	"bytes"
	_ "embed"
	"encoding/json"
	"fmt"
	"github.com/fhluo/hanconv/pkg/hanconv"
	"github.com/fhluo/hanconv/pkg/trie"
	"github.com/pelletier/go-toml/v2"
	"github.com/samber/lo"
	"golang.org/x/exp/slog"
	"log"
	"os"
	"path"
	"path/filepath"
	"strings"
	"text/template"
)

func init() {
	log.SetFlags(0)
}

var (
	//go:embed templates/conv.tmpl
	convTmplStr string
	convTmpl    = template.Must(template.New("").Parse(convTmplStr))

	//go:embed templates/cmd.tmpl
	cmdTmplStr string
	cmdTmpl    = template.Must(template.New("").Parse(cmdTmplStr))

	//go:embed config.toml
	configData []byte
	genConfig  GenConfig
)

func main() {
	cc, err := NewOpenCC()
	if err != nil {
		slog.Error("", err)
		os.Exit(1)
	}

	configs, err := cc.Configs()
	if err != nil {
		slog.Error("无法获取配置文件名", err)
		os.Exit(1)
	}

	wd, err := os.Getwd()
	if err != nil {
		slog.Error("无法获取工作目录", err)
		os.Exit(1)
	}

	dir := wd
	for filepath.Base(dir) != "hanconv" && filepath.Dir(dir) != dir {
		dir = filepath.Dir(dir)
	}

	if filepath.Dir(dir) == dir {
		slog.Error("", fmt.Errorf("无法找到 hanconv 文件夹"), "工作目录", wd)
		os.Exit(1)
	}

	var converters []*hanconv.Converter

	for _, filename := range configs {
		config, err := cc.ReadConfig(filename)
		if err != nil {
			slog.Error("无法读取配置", err, "config", filename)
		}

		conv := hanconv.New(
			strings.TrimSuffix(path.Base(filename), path.Ext(filename)),
			lo.Map(config.ConversionChain, func(conversion Conversion, _ int) *trie.Trie {
				dictionaries := lo.Map(conversion.Dictionary.Files(), func(stem string, _ int) map[string]string {
					dictionary, err := cc.ReadDictionaryByStem(stem)
					if err != nil {
						slog.Error("无法读取字典", err)
						os.Exit(1)
					}
					return dictionary
				})
				return trie.FromMap(dictionaries...)
			})...,
		)
		converters = append(converters, conv)
	}

	for _, conv := range converters {
		data, err := json.Marshal(conv)
		if err != nil {
			slog.Error("将 Converter 序列化为 JSON 失败", err)
			os.Exit(1)
		}

		if err = os.MkdirAll(filepath.Join(dir, "pkg", conv.Name), 0660); err != nil {
			slog.Error("", err)
			os.Exit(1)
		}

		if err = os.WriteFile(filepath.Join(dir, "pkg", conv.Name, conv.Name+".json"), data, 0666); err != nil {
			slog.Error("", err)
			os.Exit(1)
		}

		buffer := new(bytes.Buffer)
		err = convTmpl.Execute(buffer, map[string]string{
			"packageName": conv.Name,
			"dictionary":  conv.Name + ".json",
		})
		if err != nil {
			slog.Error("", err)
			os.Exit(1)
		}

		if err = os.WriteFile(filepath.Join(dir, "pkg", conv.Name, conv.Name+".go"), buffer.Bytes(), 0666); err != nil {
			slog.Error("", err)
			os.Exit(1)
		}
	}

	err = toml.Unmarshal(configData, &genConfig)
	if err != nil {
		slog.Error("", err)
		os.Exit(1)
	}

	convertersConfig := genConfig.ConvertersMap()

	for _, conv := range converters {
		buffer := new(bytes.Buffer)
		err = cmdTmpl.Execute(buffer, map[string]string{
			"name":        conv.Name,
			"packageName": conv.Name,
			"use":         conv.Name,
			"short":       convertersConfig[conv.Name].ConversionString(),
		})
		if err != nil {
			slog.Error("", err)
			os.Exit(1)
		}

		if err = os.WriteFile(filepath.Join(dir, "cmd", conv.Name+".go"), buffer.Bytes(), 0666); err != nil {
			slog.Error("", err)
			os.Exit(1)
		}
	}
}
