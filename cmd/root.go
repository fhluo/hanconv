package cmd

import (
	"github.com/fhluo/hanzi-conv/pkg/hanzi"
	"github.com/spf13/cobra"
	"io"
	"log"
	"os"
	"unsafe"
)

var (
	converter *hanzi.Converter

	inputFilename  string
	outputFilename string

	rootCmd = &cobra.Command{
		Use:   "hanzi",
		Short: "汉字转换工具",
		Run: func(cmd *cobra.Command, args []string) {
			var (
				input  = os.Stdin
				output = os.Stdout
				err    error
			)
			if cmd.Flags().Changed("input") {
				input, err = os.Open(inputFilename)
				if err != nil {
					log.Fatalln(err)
				}
				defer func() {
					if err := input.Close(); err != nil {
						return
					}
				}()
			}

			if cmd.Flags().Changed("output") {
				output, err = os.OpenFile(outputFilename, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, os.ModePerm)
				if err != nil {
					log.Fatalln(err)
				}
				defer func() {
					if err := output.Close(); err != nil {
						return
					}
				}()
			}

			if err := convert(input, output); err != nil {
				log.Fatalln(err)
			}
		},
	}
)

func init() {
	log.SetFlags(0)

	rootCmd.AddCommand(genCmd)

	rootCmd.Flags().StringVarP(&inputFilename, "input", "i", "", "输入文件名")
	rootCmd.Flags().StringVarP(&outputFilename, "output", "o", "", "输出文件名")
}

// convert 将源文件转化后输出到目标文件
func convert(input *os.File, output *os.File) error {
	data, err := io.ReadAll(input)
	if err != nil {
		return err
	}

	result := converter.Convert(unsafe.String(unsafe.SliceData(data), len(data)))

	if _, err = io.WriteString(output, result); err != nil {
		return err
	}

	return nil
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatalln(err)
	}
}
