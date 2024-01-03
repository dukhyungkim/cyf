package main

import (
	"database/sql"
	"encoding/json"
	"github.com/go-chi/render"
	"net/http"
	"strconv"
	"strings"
)

func GetImages(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		images, err := FetchImages(db)
		if err != nil {
			_ = render.Render(w, r, ErrInternalServerError(err))
			return
		}

		indent := r.URL.Query().Get("indent")
		data, err := MarshalJSON(images, indent)
		if err != nil {
			_ = render.Render(w, r, ErrRender(err))
			return
		}

		_, _ = w.Write(data)
	}
}

func PostImage(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		image := Image{}
		if err := render.Bind(r, &image); err != nil {
			_ = render.Render(w, r, ErrInvalidRequest(err))
			return
		}

		err := SaveImage(db, image)
		if err != nil {
			_ = render.Render(w, r, ErrInternalServerError(err))
			return
		}

		indent := r.URL.Query().Get("indent")
		data, err := MarshalJSON(image, indent)
		if err != nil {
			_ = render.Render(w, r, ErrRender(err))
			return
		}

		_, _ = w.Write(data)
	}
}

func MarshalJSON(data any, indent string) ([]byte, error) {
	if indent != "" {
		count, err := strconv.Atoi(indent)
		if err != nil {
			return nil, err
		}

		const space = " "
		spaces := make([]string, count)
		for i := 0; i < count; i++ {
			spaces[i] = space
		}
		indentStr := strings.Join(spaces, "")

		return json.MarshalIndent(data, "", indentStr)
	}

	return json.Marshal(data)
}
