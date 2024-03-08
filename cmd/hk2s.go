package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/hk2s"
	"github.com/spf13/cobra"
)

var hk2sCmd = &cobra.Command{
	Use:   "hk2s",
	Short: "繁体中文（香港） -> 简体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, hk2s.Convert)
	},
}

func init() {
	rootCmd.AddCommand(hk2sCmd)
}
