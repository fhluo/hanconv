package cmd

import (
	"fmt"
	"github.com/fhluo/hanzi-conv/pkg/trie"
	"github.com/goccy/go-json"
	"github.com/spf13/cobra"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
)

func init() {
	genCmd.Flags().StringVarP(&outputFilename, "out", "o", "", "输出文件名")
}

var genCmd = &cobra.Command{
	Use:   "gen",
	Short: "从文本文件中生成以 JSON 格式的字典文件",
	Run: func(cmd *cobra.Command, args []string) {
		var wg sync.WaitGroup

		filenames := make([]string, 0, len(args))
		for _, arg := range args {
			r, err := filepath.Glob(arg)
			if err != nil {
				log.Println(err)
				continue
			}
			filenames = append(filenames, r...)
		}

		wg.Add(len(filenames))

		for _, filename := range filenames {
			filename := filename
			go func() {
				defer wg.Done()
				generate(filename)
			}()
		}

		wg.Wait()
	},
}

// LoadDict 从指定文件中读取字典
func LoadDict(filename string) (map[string]string, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	text := string(data)

	// 将文本按行分割
	lines := regexp.MustCompile(`\r?\n`).Split(text, -1)
	spacesRE := regexp.MustCompile(`\s`)

	dict := make(map[string]string)
	for _, line := range lines {
		// 将一行按空格分割，格式符合要求则加入字典
		items := spacesRE.Split(strings.TrimSpace(line), -1)
		if len(items) >= 2 {
			dict[items[0]] = items[1]
		}
	}

	return dict, nil
}

func generate(filename string) {
	dict, err := LoadDict(filename)
	if err != nil {
		fmt.Println(err)
		return
	}
	
	data, err := json.Marshal(trie.FromMap(dict))
	if err != nil {
		fmt.Println(err)
		return
	}

	if outputFilename == "" {
		ext := filepath.Ext(filename)
		filename = filename[:len(filename)-len(ext)] + ".go"
	} else {
		filename = outputFilename
	}

	err = os.WriteFile(filename, data, 0666)
	if err != nil {
		fmt.Println(err)
		return
	}
}
