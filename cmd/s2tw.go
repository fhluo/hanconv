package cmd

import (
	"github.com/fhluo/hanconv/pkg/s2tw"
	"github.com/spf13/cobra"
)

var s2twCmd = &cobra.Command{
	Use:   "s2tw",
	Short: "s2tw",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2tw.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2twCmd)
}
