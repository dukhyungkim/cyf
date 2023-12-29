package main

type NameScore struct {
	Name      string `json:"name"`
	HighScore int32  `json:"high_score"`
}

func NewScore(name string, score int32) NameScore {
	return NameScore{name, score}
}
