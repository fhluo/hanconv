package main

import (
	"github.com/fhluo/gocc/pkg/cmd/jp2t"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	jp2t.Execute()
}
