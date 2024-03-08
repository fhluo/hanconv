package main

import (
	"github.com/fhluo/gocc/pkg/cmd/tw2t"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	tw2t.Execute()
}
