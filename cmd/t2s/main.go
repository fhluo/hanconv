package main

import (
	"github.com/fhluo/gocc/cmd"
	"log"
)

func init() {
	log.SetFlags(0)
}

func Execute() {
	if err := cmd.T2SCmd.Execute(); err != nil {
		log.Fatalln(err)
	}
}

func main() {
	Execute()
}
