package main

import (
	"crypto/sha256"
	"crypto/subtle"
	"golang.org/x/time/rate"
	"net/http"
	"os"
)

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

func reteLimitMiddleware(next http.HandlerFunc) http.HandlerFunc {
	var limiter = rate.NewLimiter(100, 30)
	return func(w http.ResponseWriter, r *http.Request) {
		if !limiter.Allow() {
			status := http.StatusTooManyRequests
			http.Error(w, http.StatusText(status), status)
			return
		}
		next.ServeHTTP(w, r)
	}
}
