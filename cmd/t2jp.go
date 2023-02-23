package cmd

import (
	"github.com/fhluo/hanconv/pkg/t2jp"
	"github.com/spf13/cobra"
)

var t2jpCmd = &cobra.Command{
	Use:   "t2jp",
	Short: "t2jp",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2jp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(t2jpCmd)
}
