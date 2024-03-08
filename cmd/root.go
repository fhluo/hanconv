package cmd

import (
	"github.com/spf13/cobra"
	"log"
)

var (
	inputFilename  string
	outputFilename string

	rootCmd = &cobra.Command{
		Use:   "gocc",
		Short: "汉字转换工具",
	}
)

func init() {
	log.SetFlags(0)

	rootCmd.PersistentFlags().StringVarP(&inputFilename, "input", "i", "", "输入文件名")
	rootCmd.PersistentFlags().StringVarP(&outputFilename, "output", "o", "", "输出文件名")
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatalln(err)
	}
}
