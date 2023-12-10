package main

import (
	"encoding/binary"
	"slices"
)

func parseBinary(file []byte) ([]NameScore, error) {
	BigEndian := []byte{0xFE, 0xFF}

	endian := file[:2]
	if slices.Compare(endian, BigEndian) == 0 {
		return readBE(file[2:])
	}
	return readLE(file[2:])
}

const sizeOfInt32 = 4

func readBE(contents []byte) ([]NameScore, error) {
	return readBinaryFunc(contents, func(fourBytes []byte) int32 {
		return int32(binary.BigEndian.Uint32(fourBytes))
	})
}

func readLE(contents []byte) ([]NameScore, error) {
	return readBinaryFunc(contents, func(fourBytes []byte) int32 {
		return int32(binary.LittleEndian.Uint32(fourBytes))
	})
}

func readBinaryFunc(contents []byte, i32Parser func(fourBytes []byte) int32) ([]NameScore, error) {
	var nameScores []NameScore

	idx := 0
	for idx < len(contents) {
		score := i32Parser(contents[idx : idx+sizeOfInt32])
		idx += sizeOfInt32

		name := bytesToString(contents[idx:])
		idx += len(name) + 1

		nameScores = append(nameScores, NameScore{
			Name:      name,
			HighScore: score,
		})
	}

	return nameScores, nil
}

func bytesToString(bytes []byte) string {
	nullIndex := slices.Index(bytes, 0x00)
	return string(bytes[:nullIndex])
}
