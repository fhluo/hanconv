package cmd

import (
	"github.com/fhluo/hanconv/pkg/t2s"
	"github.com/spf13/cobra"
)

var t2sCmd = &cobra.Command{
	Use:   "t2s",
	Short: "t2s",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2sCmd)
}
