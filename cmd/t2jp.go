package cmd

import (
	"github.com/fhluo/gocc/pkg/t2jp"
	"github.com/spf13/cobra"
)

var t2jpCmd = &cobra.Command{
	Use:   "t2jp",
	Short: "繁体中文 -> 日文汉字（新字体）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2jp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2jpCmd)
}
