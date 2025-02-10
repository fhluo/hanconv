package main

import (
	"github.com/fhluo/hanconv/go/pkg/cmd/tw2sp"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	tw2sp.Execute()
}
