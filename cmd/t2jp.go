package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/t2jp"
	"github.com/spf13/cobra"
)

var T2JPCmd = &cobra.Command{
	Use:   "t2jp",
	Short: "繁体中文 -> 日文汉字（新字体）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2jp.Convert)
	},
}

func init() {
	T2JPCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    T2JPCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, T2JPCmd)
}
