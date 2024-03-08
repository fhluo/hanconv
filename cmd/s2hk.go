package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/s2hk"
	"github.com/spf13/cobra"
)

var S2HKCmd = &cobra.Command{
	Use:   "s2hk",
	Short: "简体中文 -> 繁体中文（香港）",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2hk.Convert)
	},
}

func init() {
	S2HKCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    S2HKCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, S2HKCmd)
}
