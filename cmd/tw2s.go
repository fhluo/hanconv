package cmd

import (
	"github.com/fhluo/gocc/pkg/tw2s"
	"github.com/spf13/cobra"
)

var tw2sCmd = &cobra.Command{
	Use:   "tw2s",
	Short: "繁体中文（台湾） -> 简体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(tw2sCmd)
}
