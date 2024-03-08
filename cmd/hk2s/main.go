package main

import (
	"github.com/fhluo/gocc/pkg/cmd/hk2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	hk2s.Execute()
}
