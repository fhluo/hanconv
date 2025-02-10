package main

import (
	"github.com/samber/lo"
	"strings"
)

type Converter struct {
	Name       string   `toml:"name"`
	Conversion []string `toml:"conversion"`
}

func (c Converter) ConversionString() string {
	if len(c.Conversion) != 2 {
		panic("")
	}
	return strings.Join(c.Conversion, " -> ")
}

type GenConfig struct {
	Converters []Converter `toml:"converters"`
}

func (c GenConfig) ConvertersMap() map[string]Converter {
	return lo.SliceToMap(c.Converters, func(conv Converter) (string, Converter) {
		return conv.Name, conv
	})
}
