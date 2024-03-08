package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/t2tw"
	"github.com/spf13/cobra"
)

var t2twCmd = &cobra.Command{
	Use:   "t2tw",
	Short: "繁体中文 -> 繁体中文(台湾)",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2tw.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2twCmd)
}
