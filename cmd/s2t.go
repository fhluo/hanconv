package cmd

import (
	"github.com/fhluo/hanconv/pkg/s2t"
	"github.com/spf13/cobra"
)

var s2tCmd = &cobra.Command{
	Use:   "s2t",
	Short: "简体中文 -> 繁体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2t.Convert)
	},
}

func init() {
	rootCmd.AddCommand(s2tCmd)
}
