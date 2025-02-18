package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/t2jp"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2jp.Execute()
}
