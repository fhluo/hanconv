package cmd

import (
	"github.com/fhluo/hanconv/pkg/t2tw"
	"github.com/spf13/cobra"
)

var t2twCmd = &cobra.Command{
	Use:   "t2tw",
	Short: "t2tw",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2tw.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2twCmd)
}
