package main

import (
	"github.com/fhluo/gocc/pkg/cmd/t2jp"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2jp.Execute()
}
