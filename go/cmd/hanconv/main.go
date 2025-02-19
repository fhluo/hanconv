package main

import (
	"io"
	"log"
	"log/slog"
	"os"
	"unsafe"

	hanconv "github.com/fhluo/hanconv/go"
	"github.com/spf13/cobra"
)

var (
	inputFilename  string
	outputFilename string
	rootCmd        = &cobra.Command{
		Use:   "hanconv",
		Short: "Convert between Chinese characters variants",
	}
)

func Run(convert func(s string) string) func(cmd *cobra.Command, args []string) error {
	return func(cmd *cobra.Command, args []string) error {
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
					slog.Warn("Error closing input file", "err", err)
				}
			}()
		}

		if cmd.Flags().Changed("output") {
			output, err = os.Create(outputFilename)
			if err != nil {
				return err
			}
			defer func() {
				if err = output.Close(); err != nil {
					slog.Warn("Error closing output file", "err", err)
				}
			}()
		}

		data, err := io.ReadAll(input)
		if err != nil {
			return err
		}

		_, err = output.WriteString(convert(unsafe.String(unsafe.SliceData(data), len(data))))
		return err
	}
}

func init() {
	log.SetFlags(0)

	rootCmd.PersistentFlags().StringVarP(&inputFilename, "input", "i", "", "input filename")
	rootCmd.PersistentFlags().StringVarP(&outputFilename, "output", "o", "", "output filename")

	rootCmd.AddCommand(
		&cobra.Command{
			Use:   "s2t",
			Short: "Convert Simplified Chinese to Traditional Chinese",
			RunE:  Run(hanconv.S2T),
		},
		&cobra.Command{
			Use:   "t2s",
			Short: "Convert Traditional Chinese to Simplified Chinese",
			RunE:  Run(hanconv.T2S),
		},
		&cobra.Command{
			Use:   "s2tw",
			Short: "Convert Simplified Chinese to Traditional Chinese (Taiwan)",
			RunE:  Run(hanconv.S2TW),
		},
		&cobra.Command{
			Use:   "tw2s",
			Short: "Convert Traditional Chinese (Taiwan) to Simplified Chinese",
			RunE:  Run(hanconv.TW2S),
		},
		&cobra.Command{
			Use:   "s2twp",
			Short: "Convert Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom",
			RunE:  Run(hanconv.S2TWP),
		},
		&cobra.Command{
			Use:   "tw2sp",
			Short: "Convert Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom",
			RunE:  Run(hanconv.TW2SP),
		},
		&cobra.Command{
			Use:   "t2tw",
			Short: "Convert Traditional Chinese to Traditional Chinese (Taiwan)",
			RunE:  Run(hanconv.T2TW),
		},
		&cobra.Command{
			Use:   "tw2t",
			Short: "Convert Traditional Chinese (Taiwan) to Traditional Chinese",
			RunE:  Run(hanconv.TW2T),
		},
		&cobra.Command{
			Use:   "s2hk",
			Short: "Convert Simplified Chinese to Traditional Chinese (Hong Kong)",
			RunE:  Run(hanconv.S2HK),
		},
		&cobra.Command{
			Use:   "hk2s",
			Short: "Convert Traditional Chinese (Hong Kong) to Simplified Chinese",
			RunE:  Run(hanconv.HK2S),
		},
		&cobra.Command{
			Use:   "t2hk",
			Short: "Convert Traditional Chinese to Traditional Chinese (Hong Kong)",
			RunE:  Run(hanconv.T2HK),
		},
		&cobra.Command{
			Use:   "hk2t",
			Short: "Convert Traditional Chinese (Hong Kong) to Traditional Chinese",
			RunE:  Run(hanconv.HK2T),
		},
		&cobra.Command{
			Use:   "t2jp",
			Short: "Convert Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)",
			RunE:  Run(hanconv.T2JP),
		},
		&cobra.Command{
			Use:   "jp2t",
			Short: "Convert New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)",
			RunE:  Run(hanconv.JP2T),
		},
	)
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		slog.Error(err.Error())
		os.Exit(1)
	}
}
