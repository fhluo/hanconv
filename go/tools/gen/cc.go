package main

import (
	"fmt"
	"github.com/bytedance/sonic"
	"github.com/go-git/go-billy/v5"
	"github.com/go-git/go-billy/v5/memfs"
	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/storage/memory"
	"github.com/samber/lo"
	"io"
	"iter"
	"os"
	"path"
	"slices"
	"strings"
)

const (
	ConfigDir     = "data/config"
	DictionaryDir = "data/dictionary"
)

// OpenCC 用于获取 OpenCC 的配置和字典
type OpenCC struct {
	billy.Filesystem
}

// NewOpenCC 克隆 OpenCC 到内存和指定的文件系统，返回 OpenCC
func NewOpenCC() (*OpenCC, error) {
	o := memfs.New()

	// 克隆 master 分支，最新的提交
	_, err := git.Clone(memory.NewStorage(), o, &git.CloneOptions{
		URL:           "https://github.com/BYVoid/OpenCC",
		ReferenceName: plumbing.Master,
		SingleBranch:  true,
		Depth:         1,
		Progress:      os.Stdout,
	})

	if err != nil {
		return nil, fmt.Errorf("克隆 OpenCC 失败：%w", err)
	}

	return &OpenCC{Filesystem: o}, nil
}

// Configs 返回所有配置文件名
func (o *OpenCC) Configs() (iter.Seq[string], error) {
	configs, err := o.ReadDir(ConfigDir)
	if err != nil {
		return nil, err
	}

	return func(yield func(string) bool) {
		for info := range slices.Values(configs) {
			if path.Ext(info.Name()) == ".json" {
				if !yield(path.Join(ConfigDir, info.Name())) {
					return
				}
			}
		}
	}, nil
}

// ReadConfig 读取配置
func (o *OpenCC) ReadConfig(filename string) (config Config, err error) {
	data, err := o.ReadFile(filename)
	if err != nil {
		return
	}

	err = sonic.Unmarshal(data, &config)
	return
}

// Dictionaries 返回所有字典文件名
func (o *OpenCC) Dictionaries() (iter.Seq[string], error) {
	dictionaries, err := o.ReadDir(DictionaryDir)
	if err != nil {
		return nil, err
	}

	return func(yield func(string) bool) {
		for info := range slices.Values(dictionaries) {
			if path.Ext(info.Name()) == ".txt" {
				if !yield(path.Join(DictionaryDir, info.Name())) {
					return
				}
			}
		}
	}, nil
}

// ReadFile 读取文件
func (o *OpenCC) ReadFile(filename string) ([]byte, error) {
	f, err := o.Open(filename)
	if err != nil {
		return nil, err
	}
	defer func() {
		_ = f.Close()
	}()

	return io.ReadAll(f)
}

type Dictionary struct {
	Type         string       `json:"type"`
	File         *string      `json:"file,omitempty"`
	Dictionaries []Dictionary `json:"dicts,omitempty"`
}

func (d Dictionary) Files() []string {
	switch d.Type {
	case "ocd2":
		if path.Ext(*d.File) != ".ocd2" {
			panic(fmt.Sprintf("文件后缀应为 .ocd2：%s", path.Ext(*d.File)))
		}

		switch file := strings.TrimSuffix(*d.File, ".ocd2"); file {
		case "TWPhrases":
			return []string{"TWPhrasesIT", "TWPhrasesName", "TWPhrasesOther"}
		case "TWPhrasesRev":
			return []string{"TWPhrasesITRev", "TWPhrasesNameRev", "TWPhrasesOtherRev"}
		default:
			return []string{file}
		}
	case "group":
		return lo.FlatMap(d.Dictionaries, func(dict Dictionary, _ int) []string {
			return dict.Files()
		})
	default:
		panic(fmt.Sprintf("未知字典类型：%s", d.Type))
	}
}

type Segmentation struct {
	Type       string     `json:"type"`
	Dictionary Dictionary `json:"dict"`
}

type Conversion struct {
	Dictionary Dictionary `json:"dict"`
}

type Config struct {
	Name            string       `json:"name"`
	Segmentation    Segmentation `json:"segmentation"`
	ConversionChain []Conversion `json:"conversion_chain"`
}
