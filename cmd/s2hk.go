package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/s2hk"
	"github.com/spf13/cobra"
)

var s2hkCmd = &cobra.Command{
	Use:   "s2hk",
	Short: "简体中文 -> 繁体中文（香港）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2hk.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2hkCmd)
}
