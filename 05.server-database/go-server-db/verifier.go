package main

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"github.com/go-chi/render"
	"log"
	"net/http"
	"time"
)

func VerifyImage(db *sql.DB, image Image) render.Renderer {
	if image.AltText == "" {
		err := errors.New("alt_text cannot be empty")
		return ErrInvalidRequest(err)
	}

	isDup, err := IsDuplicated(db, image)
	if err != nil {
		return ErrInternalServerError(err)
	}
	if isDup {
		err = errors.New("duplicate image")
		return ErrInvalidRequest(err)
	}

	err = VerifyURL(image.URL)
	if err != nil {
		wrapErr := fmt.Errorf("faild to verify url: %w", err)
		return ErrInvalidRequest(wrapErr)
	}

	return nil
}

func isImageContentType(contentType string) bool {
	return contentType == "image/jpeg" ||
		contentType == "image/png" ||
		contentType == "image/gif" ||
		contentType == "image/webp" ||
		contentType == "image/svg+xml" ||
		contentType == "image/bmp" ||
		contentType == "image/tiff"
}

func VerifyURL(url string) error {
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to perform request: %w", err)
	}
	defer func() {
		if err = resp.Body.Close(); err != nil {
			log.Println("failed to close body")
		}
	}()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	contentType := resp.Header.Get("Content-Type")
	if !isImageContentType(contentType) {
		return fmt.Errorf("response is not an image, Content-Type: %s", contentType)
	}

	return nil
}
