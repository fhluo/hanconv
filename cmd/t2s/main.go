package main

import (
	"github.com/fhluo/gocc/pkg/cmd/t2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2s.Execute()
}
