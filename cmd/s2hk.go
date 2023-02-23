package cmd

import (
	"github.com/fhluo/hanconv/pkg/s2hk"
	"github.com/spf13/cobra"
)

var s2hkCmd = &cobra.Command{
	Use:   "s2hk",
	Short: "s2hk",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2hk.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2hkCmd)
}
