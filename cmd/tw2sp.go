package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/tw2sp"
	"github.com/spf13/cobra"
)

var TW2SPCmd = &cobra.Command{
	Use:   "tw2sp",
	Short: "繁体中文（台湾） -> 简体中文（转换常用词汇）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2sp.Convert)
	},
}

func init() {
	TW2SPCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    TW2SPCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, TW2SPCmd)
}
