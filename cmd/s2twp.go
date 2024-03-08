package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/s2twp"
	"github.com/spf13/cobra"
)

var s2twpCmd = &cobra.Command{
	Use:   "s2twp",
	Short: "简体中文 -> 繁体中文（台湾）（转换常用词汇）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2twp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2twpCmd)
}
