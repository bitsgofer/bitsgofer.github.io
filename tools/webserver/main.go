package main

import (
	"flag"
	"log"
	"net/http"
	"path/filepath"
)

var (
	listenAddress string
	contentDir string
)

func main() {
	// flags
	flag.StringVar(&listenAddress, "listenAddress", ":7777", "webserver bind address")
	flag.StringVar(&contentDir, "contentDir", "_html", "/path/to/html")

	// server
	fileServer := http.FileServer(http.Dir(contentDir))

	http.Handle("/", addHeaders(fileServer))
	log.Println("Running game server, listening to :7777...")
	err := http.ListenAndServe(listenAddress, nil)
	if err != nil {
		log.Fatal(err)
	}
}

func addHeaders(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Add custom headers
		// w.Header().Set("Cross-Origin-Opener-Policy", "same-origin")
		// w.Header().Set("Cross-Origin-Embedder-Policy", "require-corp")

		requestedFile := r.URL.Path
		fullPath := filepath.Join(contentDir, requestedFile)
		contentType := getContentType(fullPath)
		log.Println(contentType)
		// w.Header().Set("Content-Type", "")

		// Call the original handler
		handler.ServeHTTP(w, r)
	})
}

// getContentType returns the appropriate Content-Type header based on the file's extension.
func getContentType(filename string) string {
	switch filepath.Ext(filename) {
	case ".css":
		return "text/css"
	case ".js":
		return "application/javascript"
	case ".jpg", ".jpeg":
		return "image/jpeg"
	case ".png":
		return "image/png"
	case ".pdf":
		return "application/pdf"
	// default:
	// 	return "text/html"
	}

	return "text/html"
}
