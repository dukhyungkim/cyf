package main

import (
	"database/sql"
	"fmt"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"log"
	"net/http"
	"os"
)

func main() {
	databaseURL := os.Getenv("DATABASE_URL")
	if databaseURL == "" {
		databaseURL = "postgres://localhost:5432/server-database"
	}
	db, err := InitDatabase(databaseURL)
	if err != nil {
		log.Fatalln(err)
	}
	defer func() {
		if err := db.Close(); err != nil {
			log.Printf("failed to close database connection: %v\n", err)
		}
	}()

	fetchImages(db)

	r := chi.NewRouter()
	r.Use(middleware.Logger)

	r.Get("/images.json", HandleImages)

	const addr = ":8080"
	fmt.Printf("Listen and serve: %s\n", addr)
	if err = http.ListenAndServe(addr, r); err != nil {
		log.Fatalln(err)
	}
}

func fetchImages(db *sql.DB) ([]Image, error) {

}
