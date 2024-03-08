package main

import (
	"github.com/fhluo/gocc/pkg/cmd/t2hk"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2hk.Execute()
}
