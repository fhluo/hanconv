package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/t2tw"
	"github.com/spf13/cobra"
)

var T2TWCmd = &cobra.Command{
	Use:   "t2tw",
	Short: "繁体中文 -> 繁体中文(台湾)",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2tw.Convert)
	},
}

func init() {
	T2TWCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    T2TWCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, T2TWCmd)
}
