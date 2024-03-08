package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/jp2t"
	"github.com/spf13/cobra"
)

var jp2tCmd = &cobra.Command{
	Use:   "jp2t",
	Short: "日文汉字（新字体） -> 繁体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, jp2t.Convert)
	},
}

func init() {
	rootCmd.AddCommand(jp2tCmd)
}
