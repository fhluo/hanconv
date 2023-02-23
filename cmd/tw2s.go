package cmd

import (
	"github.com/fhluo/hanconv/pkg/tw2s"
	"github.com/spf13/cobra"
)

var tw2sCmd = &cobra.Command{
	Use:   "tw2s",
	Short: "tw2s",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(tw2sCmd)
}
