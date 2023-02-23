package cmd

import (
	"github.com/fhluo/hanconv/pkg/hk2t"
	"github.com/spf13/cobra"
)

var hk2tCmd = &cobra.Command{
	Use:   "hk2t",
	Short: "hk2t",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, hk2t.Convert)
	},
}

func init() {
	rootCmd.AddCommand(hk2tCmd)
}
