package main

import (
	"strconv"
	"strings"
)

func parseCSV(file []byte) ([]NameScore, error) {
	contents := string(file)
	rows := strings.Split(contents, "\n")

	dataLen := len(rows) - 2
	nameScores := make([]NameScore, dataLen)
	for i, row := range rows[1:] {
		columns := strings.Split(row, ",")
		if len(columns) < 2 {
			continue
		}

		score, _ := strconv.Atoi(columns[1])
		nameScores[i] = NameScore{
			Name:      columns[0],
			HighScore: int32(score),
		}
	}
	return nameScores, nil
}
