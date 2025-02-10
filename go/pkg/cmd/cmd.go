package cmd

import (
	"github.com/spf13/cobra"
	"io"
	"log"
	"os"
)

var (
	inputFilename  string
	outputFilename string
	Commands       []*cobra.Command
)

func Run(cmd *cobra.Command, convert func(data []byte) []byte) error {
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

	if _, err = output.Write(convert(data)); err != nil {
		return err
	}

	return nil
}

func New(use string, short string, convert func(data []byte) []byte) *cobra.Command {
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
