package main

import (
	"cmp"
	"fmt"
	"log"
	"path"
	"slices"
)

type NameScore struct {
	Name      string `json:"name"`
	HighScore int32  `json:"high_score"`
}

type NameScoreReader func(filepath string) ([]NameScore, error)

func main() {
	assetDir := path.Clean("../../assets")
	for filename, reader := range map[string]NameScoreReader{
		"custom-binary-be.bin": readBinary,
		"custom-binary-le.bin": readBinary,
		"data.csv":             readCSV,
	} {
		filepath := path.Join(assetDir, filename)
		nameScores, err := reader(filepath)
		if err != nil {
			log.Fatalln(err)
		}

		fmt.Printf("read %s and results:\n", filename)
		printResult(nameScores)
		fmt.Println()
	}
}

func printResult(nameScores []NameScore) {
	slices.SortFunc(nameScores, func(a, b NameScore) int {
		return cmp.Compare(a.HighScore, b.HighScore)
	})

	highest := nameScores[len(nameScores)-1]
	lowest := nameScores[0]
	fmt.Printf("highest => name: %s, score: %d\n", highest.Name, highest.HighScore)
	fmt.Printf("highest => name: %s, score: %d\n", lowest.Name, lowest.HighScore)
}
