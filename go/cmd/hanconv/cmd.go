package main

import (
	"io"
	"log"
	"os"
	"unsafe"

	hanconv "github.com/fhluo/hanconv/go"
	"github.com/spf13/cobra"
)

var (
	inputFilename  string
	outputFilename string
	Commands       []*cobra.Command
)

func Run(cmd *cobra.Command, convert func(s string) string) error {
	var (
		input  = os.Stdin
		output = os.Stdout
		err    error
	)
	if cmd.Flags().Changed("input") {
		input, err = os.Open(inputFilename)
		if err != nil {
			return err
		}
		defer func() {
			if err = input.Close(); err != nil {
				log.Println(err)
			}
		}()
	}

	if cmd.Flags().Changed("output") {
		output, err = os.OpenFile(outputFilename, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, os.ModePerm)
		if err != nil {
			log.Fatalln(err)
		}
		defer func() {
			if err = output.Close(); err != nil {
				log.Println(err)
			}
		}()
	}

	data, err := io.ReadAll(input)
	if err != nil {
		return err
	}

	if _, err = output.WriteString(convert(unsafe.String(unsafe.SliceData(data), len(data)))); err != nil {
		return err
	}

	return nil
}

func New(use string, short string, convert func(s string) string) *cobra.Command {
	cmd := &cobra.Command{
		Use:   use,
		Short: short,
		RunE: func(cmd *cobra.Command, args []string) error {
			return Run(cmd, convert)
		},
	}

	cmd.Flags().StringVarP(&inputFilename, "input", "i", "", "input filename")
	cmd.Flags().StringVarP(&outputFilename, "output", "o", "", "output filename")
	Commands = append(Commands, cmd)

	return cmd
}

func init() {
	New(
		"s2t",
		"Convert Simplified Chinese to Traditional Chinese",
		hanconv.S2T,
	)

	New(
		"t2s",
		"Convert Traditional Chinese to Simplified Chinese",
		hanconv.T2S,
	)

	New(
		"s2tw",
		"Convert Simplified Chinese to Traditional Chinese (Taiwan)",
		hanconv.S2TW,
	)

	New(
		"tw2s",
		"Convert Traditional Chinese (Taiwan) to Simplified Chinese",
		hanconv.TW2S,
	)

	New(
		"s2twp",
		"Convert Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom",
		hanconv.S2TWP,
	)

	New(
		"tw2sp",
		"Convert Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom",
		hanconv.TW2SP,
	)

	New(
		"t2tw",
		"Convert Traditional Chinese to Traditional Chinese (Taiwan)",
		hanconv.T2TW,
	)

	New(
		"tw2t",
		"Convert Traditional Chinese (Taiwan) to Traditional Chinese",
		hanconv.TW2T,
	)

	New(
		"s2hk",
		"Convert Simplified Chinese to Traditional Chinese (Hong Kong)",
		hanconv.S2HK,
	)

	New(
		"hk2s",
		"Convert Traditional Chinese (Hong Kong) to Simplified Chinese",
		hanconv.HK2S,
	)

	New(
		"t2hk",
		"Convert Traditional Chinese to Traditional Chinese (Hong Kong)",
		hanconv.T2HK,
	)

	New(
		"hk2t",
		"Convert Traditional Chinese (Hong Kong) to Traditional Chinese",
		hanconv.HK2T,
	)

	New(
		"t2jp",
		"Convert Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)",
		hanconv.T2JP,
	)

	New(
		"jp2t",
		"Convert New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)",
		hanconv.JP2T,
	)
}
