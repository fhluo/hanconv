package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/tw2s"
	"github.com/spf13/cobra"
)

var TW2SCmd = &cobra.Command{
	Use:   "tw2s",
	Short: "繁体中文（台湾） -> 简体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2s.Convert)
	},
}

func init() {
	TW2SCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    TW2SCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, TW2SCmd)
}
