package main

import (
	"context"
	"database/sql"
	"fmt"
	_ "github.com/jackc/pgx/v5/stdlib"
	"os"
	"time"
)

func InitDatabase() (*sql.DB, error) {
	databaseURL := os.Getenv("DATABASE_URL")
	if databaseURL == "" {
		databaseURL = "postgres://localhost:5432/server-database"
	}

	db, err := sql.Open("pgx", databaseURL)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to database: %w", err)
	}

	err = db.Ping()
	if err != nil {
		return nil, fmt.Errorf("failed to ping to database: %w", err)
	}

	return db, nil
}

func FetchImages(db *sql.DB) ([]Image, error) {
	const query = "SELECT title, url, alt_text FROM public.images"

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	rows, err := db.QueryContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to select images: %w", err)
	}

	var images []Image
	for rows.Next() {
		var title, url, altText string
		err = rows.Scan(&title, &url, &altText)
		if err != nil {
			return nil, fmt.Errorf("failed to scan images: %w", err)
		}

		image := Image{
			Title:   title,
			AltText: altText,
			URL:     url,
		}
		images = append(images, image)
	}

	return images, nil
}

func SaveImage(db *sql.DB, image Image) error {
	const query = "INSERT INTO public.images(title, url, alt_text) VALUES ($1, $2, $3)"

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	_, err := db.ExecContext(ctx, query, image.Title, image.URL, image.AltText)
	if err != nil {
		return err
	}
	return nil
}

func CountImage(db *sql.DB, image Image) (int, error) {
	const query = "SELECT COUNT(*) FROM public.images WHERE title = $1 AND url = $2 AND alt_text = $3"

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	var count int
	err := db.QueryRowContext(ctx, query, image.Title, image.URL, image.AltText).
		Scan(&count)
	if err != nil {
		return 0, err
	}
	return count, nil
}

func IsDuplicated(db *sql.DB, image Image) (bool, error) {
	count, err := CountImage(db, image)
	if err != nil {
		return false, err
	}

	if count == 0 {
		return false, nil
	}
	return true, nil
}
