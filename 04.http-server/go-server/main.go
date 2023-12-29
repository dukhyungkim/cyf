package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/200", handleOK())
	http.HandleFunc("/500", handleInternalServerError())
	http.Handle("/404", http.NotFoundHandler())
	http.HandleFunc("/", handleRoot())
	http.HandleFunc("/authenticated", basicAuth(handleAuthenticated()))
	http.HandleFunc("/limited", reteLimitMiddleware(handleLimited()))

	const addr = "0.0.0.0:8080"
	fmt.Println("Listening on", addr)
	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalln(err)
	}
}
