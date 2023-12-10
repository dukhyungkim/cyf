package main

import (
	"cmp"
	"fmt"
	"log"
	"os"
	"path"
	"slices"
)

type NameScoreReader func(file []byte) ([]NameScore, error)

func main() {
	err := runApp()
	if err != nil {
		log.Fatalln(err)
	}
}

func runApp() error {
	assetDir := path.Clean("../../assets")

	list := map[string]NameScoreReader{
		"custom-binary-be.bin": parseBinary,
		"custom-binary-le.bin": parseBinary,
		"data.csv":             parseCSV,
		"json.txt":             parseJSON,
		"repeated-json.txt":    parseRepeatedJSON,
	}

	for filename, reader := range list {
		filepath := path.Join(assetDir, filename)

		file, err := os.ReadFile(filepath)
		if err != nil {
			return err
		}

		nameScores, err := reader(file)
		if err != nil {
			return err
		}

		fmt.Printf("read %s and results:\n", filename)
		printResult(nameScores)
		fmt.Println()
	}

	return nil
}

func printResult(nameScores []NameScore) {
	slices.SortFunc(nameScores, func(a, b NameScore) int {
		return cmp.Compare(a.HighScore, b.HighScore)
	})

	highest := nameScores[len(nameScores)-1]
	lowest := nameScores[0]
	fmt.Printf("highest => name: %s, score: %d\n", highest.Name, highest.HighScore)
	fmt.Printf("lowest => name: %s, score: %d\n", lowest.Name, lowest.HighScore)
}
