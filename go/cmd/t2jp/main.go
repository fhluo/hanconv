package main

import (
	"github.com/fhluo/hanconv/go/pkg/cmd/t2jp"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2jp.Execute()
}
