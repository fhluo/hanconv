package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/tw2t"
	"github.com/spf13/cobra"
)

var TW2TCmd = &cobra.Command{
	Use:   "tw2t",
	Short: "繁体中文（台湾） -> 繁体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2t.Convert)
	},
}

func init() {
	TW2TCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    TW2TCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, TW2TCmd)
}
