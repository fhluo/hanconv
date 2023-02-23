package cmd

import (
	"github.com/fhluo/hanconv/pkg/hk2s"
	"github.com/spf13/cobra"
)

var hk2sCmd = &cobra.Command{
	Use:   "hk2s",
	Short: "hk2s",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, hk2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(hk2sCmd)
}
