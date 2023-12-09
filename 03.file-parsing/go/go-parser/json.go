package main

import (
	"bytes"
	"encoding/json"
)

func parseJSON(file []byte) ([]NameScore, error) {
	var nameScores []NameScore
	err := json.Unmarshal(file, &nameScores)
	if err != nil {
		return nil, err
	}
	return nameScores, nil
}

func parseRepeatedJSON(file []byte) ([]NameScore, error) {
	var nameScores []NameScore

	const comment = 35 // #
	lines := bytes.Split(file, []byte("\n"))
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		if line[0] == comment {
			continue
		}

		var ns NameScore
		err := json.Unmarshal(line, &ns)
		if err != nil {
			return nil, err
		}
		nameScores = append(nameScores, ns)
	}

	return nameScores, nil
}
