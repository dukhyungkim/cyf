package main

import (
	"encoding/binary"
	"slices"
)

func parseBinary(file []byte) ([]NameScore, error) {
	BigEndian := []byte{0xFE, 0xFF}

	endian := file[:2]
	contents := file[2:]
	if slices.Compare(endian, BigEndian) == 0 {
		return parseBinaryFunc(contents, func(fourBytes []byte) int32 {
			return int32(binary.BigEndian.Uint32(fourBytes))
		})
	}
	return parseBinaryFunc(contents, func(fourBytes []byte) int32 {
		return int32(binary.LittleEndian.Uint32(fourBytes))
	})
}

func parseBinaryFunc(contents []byte, int32Parser func(fourBytes []byte) int32) ([]NameScore, error) {
	const sizeOfInt32 = 4

	var nameScores []NameScore

	idx := 0
	for idx < len(contents) {
		score := int32Parser(contents[idx : idx+sizeOfInt32])
		idx += sizeOfInt32

		name := bytesToString(contents[idx:])
		idx += len(name) + 1

		ns := NewScore(name, score)
		nameScores = append(nameScores, ns)
	}

	return nameScores, nil
}

func bytesToString(bytes []byte) string {
	nullIndex := slices.Index(bytes, 0x00)
	return string(bytes[:nullIndex])
}
