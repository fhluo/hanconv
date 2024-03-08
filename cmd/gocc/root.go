package main

import (
	"github.com/fhluo/gocc/cmd"
	"github.com/spf13/cobra"
	"log"
)

var (
	rootCmd = &cobra.Command{
		Use:   "gocc",
		Short: "简繁转换",
	}
)

func init() {
	log.SetFlags(0)

	for _, command := range cmd.Commands {
		rootCmd.AddCommand(command)
	}
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatalln(err)
	}
}
