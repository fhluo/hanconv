package cmd

import (
	"github.com/fhluo/gocc/pkg/cc/t2hk"
	"github.com/spf13/cobra"
)

var T2HKCmd = &cobra.Command{
	Use:   "t2hk",
	Short: "繁体中文 -> 繁体中文(香港)",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, t2hk.Convert)
	},
}

func init() {
	T2HKCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
    T2HKCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "output filename")

    Commands = append(Commands, T2HKCmd)
}
