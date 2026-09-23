package main

import (
	"bytes"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"unsafe"

	hanconv "github.com/fhluo/hanconv/go"
)

type Conversion struct {
	Name        string
	Description string
	Convert     func(s string) string
}

var conversions = []Conversion{
	{
		Name:        "s2t",
		Description: "Convert Simplified Chinese to Traditional Chinese",
		Convert:     hanconv.S2T,
	},
	{
		Name:        "t2s",
		Description: "Convert Traditional Chinese to Simplified Chinese",
		Convert:     hanconv.T2S,
	},
	{
		Name:        "s2tw",
		Description: "Convert Simplified Chinese to Traditional Chinese (Taiwan)",
		Convert:     hanconv.S2TW,
	},
	{
		Name:        "tw2s",
		Description: "Convert Traditional Chinese (Taiwan) to Simplified Chinese",
		Convert:     hanconv.TW2S,
	},
	{
		Name:        "s2twp",
		Description: "Convert Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom",
		Convert:     hanconv.S2TWP,
	},
	{
		Name:        "tw2sp",
		Description: "Convert Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom",
		Convert:     hanconv.TW2SP,
	},
	{
		Name:        "t2tw",
		Description: "Convert Traditional Chinese to Traditional Chinese (Taiwan)",
		Convert:     hanconv.T2TW,
	},
	{
		Name:        "tw2t",
		Description: "Convert Traditional Chinese (Taiwan) to Traditional Chinese",
		Convert:     hanconv.TW2T,
	},
	{
		Name:        "s2hk",
		Description: "Convert Simplified Chinese to Traditional Chinese (Hong Kong)",
		Convert:     hanconv.S2HK,
	},
	{
		Name:        "hk2s",
		Description: "Convert Traditional Chinese (Hong Kong) to Simplified Chinese",
		Convert:     hanconv.HK2S,
	},
	{
		Name:        "t2hk",
		Description: "Convert Traditional Chinese to Traditional Chinese (Hong Kong)",
		Convert:     hanconv.T2HK,
	},
	{
		Name:        "hk2t",
		Description: "Convert Traditional Chinese (Hong Kong) to Traditional Chinese",
		Convert:     hanconv.HK2T,
	},
	{
		Name:        "t2jp",
		Description: "Convert Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)",
		Convert:     hanconv.T2JP,
	},
	{
		Name:        "jp2t",
		Description: "Convert New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)",
		Convert:     hanconv.JP2T,
	},
}

var index = sync.OnceValue(func() map[string]Conversion {
	index := make(map[string]Conversion)

	for _, conversion := range conversions {
		index[conversion.Name] = conversion
	}

	return index
})

var programName = sync.OnceValue(func() string {
	return strings.TrimSuffix(filepath.Base(os.Args[0]), filepath.Ext(os.Args[0]))
})

func formatHelp(programName string, conversion string, conversions []Conversion) string {
	if programName == "" {
		programName = "hanconv"
	}

	if conversion == "" {
		conversion = "<conversion>"
	}

	buffer := new(bytes.Buffer)

	// Usage
	buffer.WriteString("Usage:\n")
	_, _ = fmt.Fprintf(buffer, "  %s %s -i <path> -o <path>\n\n", programName, conversion)

	// Conversions
	if len(conversions) > 0 {
		buffer.WriteString("Conversions:\n")
		for _, conversion := range conversions {
			_, _ = fmt.Fprintf(buffer, "  %-5s  %s\n", conversion.Name, conversion.Description)
		}
		buffer.WriteString("\n")
	}

	// Options
	buffer.WriteString("Options:\n")
	buffer.WriteString("  -i, --input  <path>   Input file path\n")
	buffer.WriteString("  -o, --output <path>   Output file path\n")
	buffer.WriteString("  -h, --help            Print help")

	return buffer.String()
}

var help = sync.OnceValue(func() string {
	return formatHelp(programName(), "", conversions)
})

var subcommandHelp = sync.OnceValue(func() string {
	return formatHelp(programName(), os.Args[1], nil)
})

const (
	ExitOk = iota
	ExitError
	ExitUsage
)

func fatal(err error) {
	_, _ = fmt.Fprintf(os.Stderr, "%s: %v\n", programName(), err)
	os.Exit(ExitError)
}

func fatalUsage(err error) {
	_, _ = fmt.Fprintf(os.Stderr, "%s: %v\n", programName(), err)
	os.Exit(ExitUsage)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println(help())
		return
	}

	if os.Args[1] == "-h" || os.Args[1] == "--help" {
		fmt.Println(help())
		return
	}

	conversion, err := parseConversion(os.Args[1])
	if err != nil {
		fatalUsage(err)
	}

	options, err := parseOptions(os.Args[2:])
	if err != nil {
		fatalUsage(err)
	}
	if options.help {
		fmt.Println(subcommandHelp())
		return
	}

	if err := convert(options.inputPath, options.outputPath, conversion.Convert); err != nil {
		fatal(err)
	}
}

func parseConversion(arg string) (Conversion, error) {
	conversion, ok := index()[arg]
	if !ok {
		return conversion, fmt.Errorf("unknown conversion %q", arg)
	}

	return conversion, nil
}

type Options struct {
	help       bool
	inputPath  string
	outputPath string
}

func parseOptions(args []string) (options Options, err error) {
	for i := 0; i < len(args); i++ {
		switch args[i] {
		case "-h", "--help":
			options.help = true
			return
		case "-i", "--input":
			if i+1 >= len(args) || args[i+1] == "" {
				err = fmt.Errorf("flag %s requires a path", args[i])
				return
			}

			options.inputPath = args[i+1]
			i++
		case "-o", "--output":
			if i+1 >= len(args) || args[i+1] == "" {
				err = fmt.Errorf("flag %s requires a path", args[i])
				return
			}

			options.outputPath = args[i+1]
			i++
		default:
			if path, ok := strings.CutPrefix(args[i], "--input="); ok {
				if path == "" {
					err = fmt.Errorf("flag --input requires a path")
					return
				}

				options.inputPath = path
				continue
			}

			if path, ok := strings.CutPrefix(args[i], "--output="); ok {
				if path == "" {
					err = fmt.Errorf("flag --output requires a path")
					return
				}

				options.outputPath = path
				continue
			}

			err = fmt.Errorf("unknown flag %q", args[i])
			return
		}
	}

	return
}

func convert(inputPath string, outputPath string, f func(s string) string) (err error) {
	var (
		input  = os.Stdin
		output = os.Stdout
	)

	if inputPath != "" {
		input, err = os.Open(inputPath)
		if err != nil {
			return fmt.Errorf("open input file: %w", err)
		}
		defer func() {
			_ = input.Close()
		}()
	}

	if outputPath != "" {
		output, err = os.Create(outputPath)
		if err != nil {
			return fmt.Errorf("open output file: %w", err)
		}
		defer func() {
			if closeErr := output.Close(); closeErr != nil {
				err = errors.Join(err, fmt.Errorf("close output file: %w", closeErr))
			}
		}()
	}

	data, err := io.ReadAll(input)
	if err != nil {
		return fmt.Errorf("read input file: %w", err)
	}

	_, err = output.WriteString(f(unsafe.String(unsafe.SliceData(data), len(data))))
	if err != nil {
		return fmt.Errorf("write output file: %w", err)
	}

	return nil
}
