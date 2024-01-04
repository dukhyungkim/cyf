package main

import (
	"database/sql"
	"encoding/json"
	"errors"
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

		if image.AltText == "" {
			err := errors.New("alt_text cannot be empty")
			_ = render.Render(w, r, ErrInvalidRequest(err))
			return
		}

		isDup, err := IsDuplicated(db, image)
		if err != nil {
			_ = render.Render(w, r, ErrInternalServerError(err))
			return
		}
		if isDup {
			err = errors.New("duplicate image")
			_ = render.Render(w, r, ErrInvalidRequest(err))
			return
		}

		err = SaveImage(db, image)
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
	if indent == "" {
		return json.Marshal(data)
	}

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
