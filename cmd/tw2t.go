package cmd

import (
	"github.com/fhluo/hanconv/pkg/tw2t"
	"github.com/spf13/cobra"
)

var tw2tCmd = &cobra.Command{
	Use:   "tw2t",
	Short: "繁体中文（台湾） -> 繁体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2t.Convert)
	},
}

func init() {
	rootCmd.AddCommand(tw2tCmd)
}
