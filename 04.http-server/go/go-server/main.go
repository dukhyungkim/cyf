package main

import (
	"crypto/sha256"
	"crypto/subtle"
	"fmt"
	"html"
	"io"
	"net/http"
	"net/url"
	"os"
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
				http.Error(w, wrapErr.Error(), http.StatusInternalServerError)
				return
			}
			result := fmt.Sprintf("<!DOCTYPE html><html>%s", html.EscapeString(string(body)))
			w.Write([]byte(result))
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

	http.HandleFunc("/authenticated", basicAuth(func(w http.ResponseWriter, r *http.Request) {
		username, _, _ := r.BasicAuth()
		result := fmt.Sprintf("<!DOCTYPE html>\n<html>\nHello %s!", username)
		w.Write([]byte(result))
	}))

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

var (
	expectedUsernameHash [32]byte
	expectedPasswordHash [32]byte
)

func init() {
	expectedUsername := os.Getenv("AUTH_USERNAME")
	if expectedUsername == "" {
		const defaultUsername = "username"
		expectedUsername = defaultUsername
	}

	expectedPassword := os.Getenv("AUTH_PASSWORD")
	if expectedPassword == "" {
		const defaultPassword = "password"
		expectedPassword = defaultPassword
	}

	expectedUsernameHash = sha256.Sum256([]byte(expectedUsername))
	expectedPasswordHash = sha256.Sum256([]byte(expectedPassword))
}

func basicAuth(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		username, password, ok := r.BasicAuth()
		if ok {
			usernameHash := sha256.Sum256([]byte(username))
			passwordHash := sha256.Sum256([]byte(password))

			usernameMatch := subtle.ConstantTimeCompare(usernameHash[:], expectedUsernameHash[:]) == 1
			passwordMatch := subtle.ConstantTimeCompare(passwordHash[:], expectedPasswordHash[:]) == 1

			if usernameMatch && passwordMatch {
				next.ServeHTTP(w, r)
				return
			}
		}

		w.Header().Set("WWW-Authenticate", `Basic realm="restricted", charset="UTF-8"`)
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
	}
}
