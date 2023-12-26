package main

import (
	"fmt"
	"golang.org/x/time/rate"
	"log"
	"net/http"
)

var limiter = rate.NewLimiter(100, 30)

func main() {
	http.HandleFunc("/200", handleOK())
	http.HandleFunc("/500", handleInternalServerError())
	http.Handle("/404", http.NotFoundHandler())
	http.HandleFunc("/", handleRoot())
	http.HandleFunc("/authenticated", basicAuth(handleAuthenticated()))

	http.HandleFunc("/limited", handleLimited())

	const addr = "0.0.0.0:8080"
	fmt.Println("Listening on", addr)
	if err := http.ListenAndServe(addr, nil); err != nil {
		log.Fatalln(err)
	}
}
