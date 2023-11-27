package cmd

import (
	"fmt"
	"os"
)

func Execute() {
	readPath := "."
	if len(os.Args) > 1 {
		readPath = os.Args[1]
	}

	if readPath == "-h" {
		printHelp(os.Args[0])
		return
	}

	stat, err := os.Stat(readPath)
	if err != nil {
		fmt.Println(err)
		return
	}

	if !stat.IsDir() {
		fmt.Println(readPath)
		return
	}
	printDir(err, readPath)
}

func printDir(err error, readPath string) {
	entries, err := os.ReadDir(readPath)
	if err != nil {
		fmt.Println(err)
		return
	}
	for _, entry := range entries {
		fmt.Println(entry.Name())
	}
}

func printHelp(name string) {
	fmt.Printf("usage: %s [file ...]\n", name)
	fmt.Println("  Options:")
	fmt.Println("  -h\t\tprint this message")
}
