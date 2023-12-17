package main

import (
	"fmt"
	"html"
	"io"
	"net/http"
	"net/url"
	"strings"
)

func handleRoot() func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			w.Header().Add("Content-Type", "text/html")
			body, err := io.ReadAll(r.Body)
			if err != nil {
				wrapErr := fmt.Errorf("failed to read request body: %w", err)
				http.Error(w, wrapErr.Error(), http.StatusInternalServerError)
				return
			}
			result := fmt.Sprintf("<!DOCTYPE html><html>%s", html.EscapeString(string(body)))
			_, _ = w.Write([]byte(result))
		case http.MethodGet:
			queryParams, err := parseQueryParams(r)
			if err != nil {
				wrapErr := fmt.Errorf("failed to parse query: %w", err)
				http.Error(w, wrapErr.Error(), http.StatusBadRequest)
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
				_, _ = w.Write([]byte(result))
				return
			}

			_, _ = w.Write([]byte("<!DOCTYPE html><html><em>Hello, world</em>"))
		default:
			status := http.StatusMethodNotAllowed
			w.WriteHeader(status)
			_, _ = w.Write([]byte(http.StatusText(status)))
		}
	}
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

func handleOK() func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		status := http.StatusOK
		result := fmt.Sprintf("%d", status)
		w.WriteHeader(status)
		_, _ = w.Write([]byte(result))
	}
}

func handleInternalServerError() func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		status := http.StatusInternalServerError
		w.WriteHeader(status)
		_, _ = w.Write([]byte(http.StatusText(status)))
	}
}

func handleAuthenticated() func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		username, _, _ := r.BasicAuth()
		result := fmt.Sprintf("<!DOCTYPE html>\n<html>\nHello %s!", username)
		_, _ = w.Write([]byte(result))
	}
}
