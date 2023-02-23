package cmd

import (
	"github.com/fhluo/hanconv/pkg/t2hk"
	"github.com/spf13/cobra"
)

var t2hkCmd = &cobra.Command{
	Use:   "t2hk",
	Short: "繁体中文 -> 繁体中文(香港)",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2hk.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2hkCmd)
}
