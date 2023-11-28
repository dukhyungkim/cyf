package main

import (
	"fmt"
	"os"
	"slices"
)

func main() {
	if slices.Contains(os.Args, "-h") {
		printHelp(os.Args[0])
		return
	}

	if len(os.Args) == 0 {
		return
	}

	filename := os.Args[1]
	file, err := os.ReadFile(filename)
	if err != nil {
		fmt.Println(err)
		return
	}

	_, err = os.Stdout.Write(file)
	if err != nil {
		fmt.Println(err)
		return
	}
}

func printHelp(name string) {
	fmt.Printf("usage: %s [file ...]\n", name)
	fmt.Println("  Options:")
	fmt.Println("  -h\t\tprint this message")
}
