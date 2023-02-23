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
	"io/fs"
	"os"
	"path"
	"regexp"
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
func (o *OpenCC) Configs() ([]string, error) {
	configs, err := o.ReadDir(ConfigDir)
	if err != nil {
		return nil, err
	}

	return lo.FilterMap(configs, func(info fs.FileInfo, _ int) (string, bool) {
		if path.Ext(info.Name()) == ".json" {
			return path.Join(ConfigDir, info.Name()), true
		}
		return "", false
	}), nil
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
func (o *OpenCC) Dictionaries() ([]string, error) {
	dictionaries, err := o.ReadDir(DictionaryDir)
	if err != nil {
		return nil, err
	}

	return lo.FilterMap(dictionaries, func(info fs.FileInfo, _ int) (string, bool) {
		if path.Ext(info.Name()) == ".txt" {
			return path.Join(DictionaryDir, info.Name()), true
		}
		return "", false
	}), nil
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

// ReadDictionary 读取字典
func (o *OpenCC) ReadDictionary(filename string) (map[string]string, error) {
	data, err := o.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	lines := regexp.MustCompile(`\r?\n`).Split(string(data), -1)

	spacesRE := regexp.MustCompile(`\s`)

	dictionary := make(map[string]string)

	for _, line := range lines {
		items := spacesRE.Split(strings.TrimSpace(line), -1)
		if len(items) >= 2 {
			dictionary[items[0]] = items[1]
		}
	}

	return dictionary, nil
}

// ReadDictionaryReverse 读取字典，交换键值
func (o *OpenCC) ReadDictionaryReverse(filename string) (map[string]string, error) {
	data, err := o.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	lines := regexp.MustCompile(`\r?\n`).Split(string(data), -1)

	spacesRE := regexp.MustCompile(`\s`)

	dictionary := make(map[string]string)

	for _, line := range lines {
		items := spacesRE.Split(strings.TrimSpace(line), -1)
		for _, item := range items[1:] {
			if item != items[0] {
				dictionary[item] = items[0]
			}
		}
	}

	return dictionary, nil
}

func (o *OpenCC) ReadDictionaryByStem(stem string) (map[string]string, error) {
	dictionaries, err := o.Dictionaries()
	if err != nil {
		return nil, err
	}

	dictionary, ok := lo.Find(dictionaries, func(dictionary string) bool {
		base := path.Base(dictionary)
		return strings.TrimSuffix(base, path.Ext(base)) == stem
	})
	if ok {
		return o.ReadDictionary(dictionary)
	}

	if !strings.HasSuffix(stem, "Rev") {
		return nil, fmt.Errorf("找不到该字典文件：%s.txt", stem)
	}

	stem = strings.TrimSuffix(stem, "Rev")
	dictionary, ok = lo.Find(dictionaries, func(dictionary string) bool {
		base := path.Base(dictionary)
		return strings.TrimSuffix(base, path.Ext(base)) == stem
	})
	if ok {
		return o.ReadDictionaryReverse(dictionary)
	}

	return nil, fmt.Errorf("找不到该字典文件：%sRev.txt", stem)
}

type Dictionary struct {
	Type         string       `json:"type"`
	File         *string      `json:"file,omitempty"`
	Dictionaries []Dictionary `json:"dicts,omitempty"`
}

func (d Dictionary) Files() []string {
	switch d.Type {
	case "ocd2":
		return []string{
			*d.File,
		}
	case "group":
		return lo.FlatMap(d.Dictionaries, func(dict Dictionary, _ int) []string {
			return dict.Files()
		})
	default:
		panic(fmt.Sprintf("未知字典类型：%s", d.Type))
	}
}

func (d Dictionary) FilesStems() []string {
	return lo.Map(d.Files(), func(file string, _ int) string {
		if path.Ext(file) == ".ocd2" {
			return strings.TrimSuffix(file, ".ocd2")
		}
		panic(fmt.Sprintf("文件后缀应为 .ocd2：%s", path.Ext(file)))
	})
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
