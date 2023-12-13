package main

import (
	"fmt"
	"html"
	"io"
	"net/http"
	"net/url"
	"strings"
)

func main() {
	http.HandleFunc("/200", func(w http.ResponseWriter, r *http.Request) {
		status := http.StatusOK
		result := fmt.Sprintf("%d", status)
		w.WriteHeader(status)
		w.Write([]byte(result))
	})

	http.HandleFunc("/500", func(w http.ResponseWriter, r *http.Request) {
		status := http.StatusInternalServerError
		w.WriteHeader(status)
		w.Write([]byte(http.StatusText(status)))
	})

	http.Handle("/404", http.NotFoundHandler())

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			w.Header().Add("Content-Type", "text/html")
			body, err := io.ReadAll(r.Body)
			if err != nil {
				wrapErr := fmt.Errorf("failed to read request body: %w", err)
				handleError(w, http.StatusInternalServerError, wrapErr)
				return
			}
			result := fmt.Sprintf("<!DOCTYPE html><html>%s", html.EscapeString(string(body)))
			w.Write([]byte(result))
		case http.MethodGet:
			queryParams, err := parseQueryParams(r)
			if err != nil {
				wrapErr := fmt.Errorf("failed to parse query: %w", err)
				handleError(w, http.StatusBadRequest, wrapErr)
				return
			}

			w.Header().Add("Content-Type", "text/html")
			if queryParams.Has("foo") {
				foo := queryParams.Get("foo")
				const template = `<!DOCTYPE html>
<html>
<em>Hello, world</em>
<p>Query parameters:
<ul>
<li>foo: %s</li>
</ul>
`
				result := fmt.Sprintf(template, html.EscapeString(foo))
				w.Write([]byte(result))
				return
			}

			w.Write([]byte("<!DOCTYPE html><html><em>Hello, world</em>"))
		default:
			status := http.StatusMethodNotAllowed
			w.WriteHeader(status)
			w.Write([]byte(http.StatusText(status)))
		}
	})

	http.ListenAndServe(":8080", nil)
}

func parseQueryParams(r *http.Request) (url.Values, error) {
	queryParams := strings.Split(r.RequestURI, "?")
	if len(queryParams) < 2 {
		return url.Values{}, nil
	}
	values, err := url.ParseQuery(queryParams[1])
	if err != nil {
		return nil, err
	}
	return values, nil
}

func handleError(w http.ResponseWriter, status int, err error) {
	w.WriteHeader(status)
	w.Write([]byte(err.Error()))
}
