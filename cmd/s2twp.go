package cmd

import (
	"github.com/fhluo/hanconv/pkg/s2twp"
	"github.com/spf13/cobra"
)

var s2twpCmd = &cobra.Command{
	Use:   "s2twp",
	Short: "s2twp",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2twp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2twpCmd)
}
