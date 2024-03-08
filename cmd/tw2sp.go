package cmd

import (
	"github.com/fhluo/gocc/pkg/tw2sp"
	"github.com/spf13/cobra"
)

var tw2spCmd = &cobra.Command{
	Use:   "tw2sp",
	Short: "繁体中文（台湾） -> 简体中文（转换常用词汇）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2sp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(tw2spCmd)
}
