package cmd

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

func Execute() {
	if slices.Contains(os.Args, "-h") {
		printHelp(os.Args[0])
		return
	}

	readPath := "."
	var opts []string
	for i := 1; i < len(os.Args); i++ {
		arg := os.Args[i]
		if strings.HasPrefix(arg, "-") {
			opts = append(opts, arg)
			continue
		}
		readPath = arg
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
	printDir(readPath, opts...)
}

func printDir(readPath string, opts ...string) {
	entries, err := os.ReadDir(readPath)
	if err != nil {
		fmt.Println(err)
		return
	}
	if len(opts) == 0 {
		for _, entry := range entries {
			fmt.Println(entry.Name())
		}
		return
	}
	switch opts[0] {
	case "-m":
		names := make([]string, len(entries))
		for i, entry := range entries {
			names[i] = entry.Name()
		}
		fmt.Println(strings.Join(names, ", "))
	}
}

func printHelp(name string) {
	fmt.Printf("usage: %s [file ...]\n", name)
	fmt.Println("  Options:")
	fmt.Println("  -h\t\tprint this message")
	fmt.Println("  -m\t\tStream output format; list files across the page, separated by commas.")
}
