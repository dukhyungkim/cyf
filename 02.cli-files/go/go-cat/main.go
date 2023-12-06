package main

import (
	"bytes"
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

	var files []string
	printLineNum := false
	for _, arg := range os.Args[1:] {
		if arg == "-n" {
			printLineNum = true
			continue
		}
		if arg[0] == '-' {
			continue
		}
		files = append(files, arg)
	}

	for _, file := range files {
		contents, err := os.ReadFile(file)
		if err != nil {
			fmt.Println(err)
			return
		}

		if printLineNum {
			printWithLineNum(contents)
			continue
		}
		fmt.Print(string(contents))
	}
}

func printWithLineNum(contents []byte) {
	lines := bytes.Split(contents, []byte("\n"))
	last := len(lines) - 1
	for i, line := range lines {
		fmt.Printf("%6d\t%s", i+1, string(line))
		if i != last {
			fmt.Println()
		}
	}
}

func printHelp(name string) {
	fmt.Printf("usage: %s [file ...]\n", name)
	fmt.Println("  Options:")
	fmt.Println("  -h\t\tprint this message")
	fmt.Println("  -n\t\tNumber the output lines, starting at 1")
}
