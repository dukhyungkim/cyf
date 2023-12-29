package main

import (
	"fmt"
	"io"
	"net/http"
	"strconv"
	"time"
)

const serverAddr = "http://localhost:8080"

func main() {
	response, err := http.Get(serverAddr)
	if err != nil {
		fmt.Printf("failed to get weather: %v\n", err)
		return
	}
	defer func() {
		if err = response.Body.Close(); err != nil {
			fmt.Printf("failed to close body: %v", err)
		}
	}()

	body, err := io.ReadAll(response.Body)
	if err != nil {
		fmt.Printf("failed to read body: %v\n", err)
		return
	}

	switch response.StatusCode {
	case http.StatusOK:
		fmt.Println(string(body))
	case http.StatusTooManyRequests:
		retryAfterHeader := response.Header.Get("Retry-After")
		retryAfter, err := strconv.Atoi(retryAfterHeader)
		if err != nil {
			timeAfterDelay, err := time.Parse(http.TimeFormat, retryAfterHeader)
			if err != nil {
				fmt.Printf("can't determine how long to sleep: %s\n", retryAfterHeader)
				return
			}
			retryAfter := timeAfterDelay.Sub(time.Now()).Seconds()
			waitAndRetry(int(retryAfter))
			return
		}

		waitAndRetry(retryAfter)
	}
}

func waitAndRetry(retryAfter int) {
	if retryAfter < 5 {
		fmt.Printf("try again after %d seconds...\n", retryAfter)
		time.Sleep(time.Duration(retryAfter) * time.Second)
		fmt.Println("retry")
	} else {
		fmt.Printf("wait too long time(%d seconds). can't get the weather.\n", retryAfter)
	}
}
