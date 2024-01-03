package main

import (
	"net/http"
)

type Image struct {
	Title   string `json:"title"`
	AltText string `json:"alt_text"`
	URL     string `json:"url"`
}

func (i *Image) Bind(_ *http.Request) error {
	return nil
}

func (i *Image) Render(_ http.ResponseWriter, _ *http.Request) error {
	return nil
}
