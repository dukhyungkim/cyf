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

func main() {
	assetDir := path.Clean("../../assets")
	for filename, reader := range map[string]func(filepath string) ([]NameScore, error){
		"custom-binary-be.bin": readBinary,
		"custom-binary-le.bin": readBinary,
	} {
		filepath := path.Join(assetDir, filename)
		nameScores, err := reader(filepath)
		if err != nil {
			log.Fatalln(err)
		}

		fmt.Println()
		printResult(nameScores)
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
