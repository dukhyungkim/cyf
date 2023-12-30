package main

import (
	"encoding/json"
	"fmt"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/render"
	"log"
	"net/http"
	"strconv"
	"strings"
)

type Image struct {
	Title   string `json:"title"`
	AltText string `json:"alt_text"`
	URL     string `json:"url"`
}

var (
	images = []Image{
		{
			Title:   "Sunset",
			AltText: "Clouds at sunset",
			URL:     "https://images.unsplash.com/photo-1506815444479-bfdb1e96c566?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1000&q=80",
		},
		{
			Title:   "Mountain",
			AltText: "A mountain at sunset",
			URL:     "https://images.unsplash.com/photo-1540979388789-6cee28a1cdc9?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1000&q=80",
		},
	}
)

func main() {
	r := chi.NewRouter()
	r.Use(middleware.Logger)

	r.Get("/images.json", HandleImages)

	const addr = ":8080"
	fmt.Printf("Listen and serve: %s\n", addr)
	if err := http.ListenAndServe(addr, r); err != nil {
		log.Fatalln(err)
	}
}

func HandleImages(w http.ResponseWriter, r *http.Request) {
	indent := r.URL.Query().Get("indent")

	var data []byte
	if indent != "" {
		count, err := strconv.Atoi(indent)
		if err != nil {
			_ = render.Render(w, r, ErrRender(err))
			return
		}

		const space = " "
		spaces := make([]string, count)
		for i := 0; i < count; i++ {
			spaces[i] = space
		}
		indentStr := strings.Join(spaces, "")

		data, err = json.MarshalIndent(images, "", indentStr)
		if err != nil {
			_ = render.Render(w, r, ErrRender(err))
			return
		}
	} else {
		var err error
		data, err = json.Marshal(images)
		if err != nil {
			_ = render.Render(w, r, ErrRender(err))
			return
		}
	}

	_, _ = w.Write(data)
}

type ErrResponse struct {
	Err            error `json:"-"`
	HTTPStatusCode int   `json:"-"`

	StatusText string `json:"status"`
	AppCode    int64  `json:"code,omitempty"`
	ErrorText  string `json:"error,omitempty"`
}

func (e *ErrResponse) Render(_ http.ResponseWriter, r *http.Request) error {
	render.Status(r, e.HTTPStatusCode)
	return nil
}

func ErrRender(err error) render.Renderer {
	return &ErrResponse{
		Err:            err,
		HTTPStatusCode: 422,
		StatusText:     "Error rendering response.",
		ErrorText:      err.Error(),
	}
}
