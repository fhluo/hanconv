package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/hk2t"
	"github.com/spf13/cobra"
)

var HK2TCmd = &cobra.Command{
	Use:   "hk2t",
	Short: "繁体中文（香港） -> 繁体中文",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, hk2t.Convert)
	},
}

func init() {
	HK2TCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    HK2TCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, HK2TCmd)
}
