package cmd

import (
	"github.com/fhluo/hanzi-conv/pkg/s2t"
	"github.com/spf13/cobra"
)

var s2tCmd = &cobra.Command{
	Use:   "s2t",
	Short: "s2t",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, s2t.Convert)
	},
}
