package main

import (
	"github.com/spf13/cobra"
	"log"
)

var (
	rootCmd = &cobra.Command{
		Use:   "hanconv",
		Short: "汉字转换",
	}
)

func init() {
	log.SetFlags(0)

	for _, command := range Commands {
		rootCmd.AddCommand(command)
	}
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatalln(err)
	}
}
