package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/t2s"
	"github.com/spf13/cobra"
)

var t2sCmd = &cobra.Command{
	Use:   "t2s",
	Short: "繁体中文 -> 简体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2sCmd)
}
