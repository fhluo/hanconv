package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/s2tw"
	"github.com/spf13/cobra"
)

var S2TWCmd = &cobra.Command{
	Use:   "s2tw",
	Short: "简体中文 -> 繁体中文（台湾）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2tw.Convert)
	},
}

func init() {
	S2TWCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    S2TWCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, S2TWCmd)
}
