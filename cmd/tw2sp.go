package cmd

import (
	"github.com/fhluo/hanconv/pkg/tw2sp"
	"github.com/spf13/cobra"
)

var tw2spCmd = &cobra.Command{
	Use:   "tw2sp",
	Short: "tw2sp",
	RunE: func(cmd *cobra.Command, args []string) error {
		return run(cmd, tw2sp.Convert)
	},
}

func init() {
	rootCmd.AddCommand(tw2spCmd)
}
