package cmd

import (
	"fmt"
	"github.com/bytedance/sonic"
	"github.com/fhluo/hanzi-conv/pkg/trie"
	"github.com/spf13/cobra"
	"os"
	"regexp"
	"strings"
)

func init() {
	genCmd.Flags().StringVarP(&outputFilename, "output", "o", "", "输出文件名")
}

var genCmd = &cobra.Command{
	Use:   "gen",
	Short: "从文本文件中生成以 JSON 格式的字典文件",
	RunE: func(cmd *cobra.Command, args []string) error {
		dictionaries := make([]map[string]string, 0, len(args))

		for _, filename := range args {
			dict, err := LoadDict(filename)
			if err != nil {
				return fmt.Errorf("could not load dictionary: %w", err)
			}
			dictionaries = append(dictionaries, dict)
		}

		data, err := sonic.Marshal(trie.FromMap(dictionaries...))
		if err != nil {
			return fmt.Errorf("failed to marshal trie to json: %w", err)
		}

		return os.WriteFile(outputFilename, data, 0666)
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
