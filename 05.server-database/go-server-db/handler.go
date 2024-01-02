package main

import (
	"encoding/json"
	"github.com/go-chi/render"
	"net/http"
	"strconv"
	"strings"
)

func HandleImages(images []Image) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
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
}
