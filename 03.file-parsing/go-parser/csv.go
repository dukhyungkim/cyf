package main

import (
	"strconv"
	"strings"
)

func parseCSV(file []byte) ([]NameScore, error) {
	const NameIndex = 0
	const ScoreIndex = 1
	const MinColumnCount = 2

	contents := string(file)
	rows := strings.Split(contents, "\n")

	dataLen := len(rows) - 2 // with Header
	nameScores := make([]NameScore, 0, dataLen)
	for _, row := range rows[1:] {
		columns := strings.Split(row, ",")
		if len(columns) < MinColumnCount {
			continue
		}

		name := columns[NameIndex]
		score, _ := strconv.Atoi(columns[ScoreIndex])

		ns := NewScore(name, int32(score))
		nameScores = append(nameScores, ns)
	}
	return nameScores, nil
}
