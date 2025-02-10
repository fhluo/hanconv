package main

import (
	"github.com/fhluo/hanconv/go/pkg/cmd"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/hk2s"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/hk2t"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/jp2t"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/s2hk"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/s2t"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/s2tw"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/s2twp"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/t2hk"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/t2jp"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/t2s"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/t2tw"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/tw2s"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/tw2sp"
	_ "github.com/fhluo/hanconv/go/pkg/cmd/tw2t"
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
