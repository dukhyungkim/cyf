package main

import (
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/200", handleOK())
	http.HandleFunc("/500", handleInternalServerError())
	http.Handle("/404", http.NotFoundHandler())
	http.HandleFunc("/", handleRoot())
	http.HandleFunc("/authenticated", basicAuth(handleAuthenticated()))

	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatalln(err)
	}
}
