package main

import (
	"fmt"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"log"
	"net/http"
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
