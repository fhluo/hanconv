package main

import (
	"github.com/fhluo/gocc/pkg/cmd/tw2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	tw2s.Execute()
}
