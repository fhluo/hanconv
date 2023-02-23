package cmd

import (
	"github.com/fhluo/hanconv/pkg/jp2t"
	"github.com/spf13/cobra"
)

var jp2tCmd = &cobra.Command{
	Use:   "jp2t",
	Short: "jp2t",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, jp2t.Convert)
	},
}

func init() {
	rootCmd.AddCommand(jp2tCmd)
}
