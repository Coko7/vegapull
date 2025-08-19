package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
)

type Pack struct {
	ID         string `json:"id"`
	RawTitle   string `json:"raw_title"`
	TitleParts struct {
		Prefix string `json:"prefix"`
		Title  string `json:"title"`
		Label  string `json:"label"`
	} `json:"title_parts"`
}

const (
	language  = "english"
	vegaData  = "data/english"
	vegaBin   = "target/release/vegapull"
	packsFile = "packs.json"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	// If data directory exists, ask for confirmation and delete it.
	if exists(vegaData) {
		log.Printf("The %s is about to be wiped to hold new data, do you want to proceed? (y/N) ", vegaData)
		confirm, err := reader.ReadString('\n')
		if err != nil {
			log.Fatalf("Input error: %v\n", err)
		}
		if !strings.HasPrefix(strings.ToLower(strings.TrimSpace(confirm)), "y") {
			log.Fatalf("Aborted by user\n")
		}
		if err := os.RemoveAll(vegaData); err != nil {
			log.Fatalf("Failed to remove directory %s: %v\n", vegaData, err)
		}
	}

	// Create new data directory.
	if err := os.MkdirAll(vegaData, 0755); err != nil {
		log.Fatalf("Failed to create directory %s: %v\n", vegaData, err)
	}
	log.Printf("Created dir: %s\n\n", vegaData)

	packs, err := getPacks()
	if err != nil {
		log.Fatalf("Failed to get packs: %v\n", err)
	}

	err = pullCards(packs)
	if err != nil {
		log.Fatalf("Failed to pull cards: %v\n", err)
	}

	err = downloadImages(packs)
	if err != nil {
		log.Fatalf("Failed to download images: %v\n", err)
	}

	fmt.Println("Successfully filled the punk records with latest data")
}

// getPacks retrieves the packs using vegapull and returns a slice of Pack structs.
func getPacks() ([]Pack, error) {
	packsPath := filepath.Join(vegaData, packsFile)
	err := runCommand(vegaBin, []string{"--language", language, "packs"}, packsPath)
	if err != nil {
		return nil, fmt.Errorf("Failed to pull packs using vegapull: %v\n", err)
	}
	data, err := os.ReadFile(packsPath)
	if err != nil {
		return nil, fmt.Errorf("error reading %s: %v", packsPath, err)
	}
	var packs []Pack
	if err := json.Unmarshal(data, &packs); err != nil {
		return nil, fmt.Errorf("error parsing %s: %v", packsPath, err)
	}
	return packs, nil
}

// pullCards loops over packs, pulling cards concurrently using a WaitGroup.
func pullCards(packs []Pack) error {
	var wg sync.WaitGroup
	errChan := make(chan error, len(packs))
	for i, pack := range packs {
		wg.Add(1)
		go func() {
			defer wg.Done()
			title := pack.RawTitle
			log.Printf("[%d/%d] VegaPulling cards for pack '%s' (%s)...", i, len(packs), title, pack.ID)
			outPath := filepath.Join(vegaData, fmt.Sprintf("cards_%s.json", pack.ID))
			err := runCommand(vegaBin, []string{"--language", language, "cards", pack.ID}, outPath)
			if err != nil {
				errChan <- fmt.Errorf("failed to pull cards for pack %s (%s): %v", title, pack.ID, err)
				return
			}
			log.Printf("[%d/%d] Successfully pulled cards for pack '%s' (%s)\n\n", i, len(packs), title, pack.ID)
		}()
	}
	wg.Wait()
	close(errChan)
	for err := range errChan {
		if err != nil {
			return err
		}
	}
	return nil
}

// downloadImages loops over packs, downloading images concurrently using a WaitGroup.
func downloadImages(packs []Pack) error {
	var wg sync.WaitGroup
	errChan := make(chan error, len(packs))
	for i, pack := range packs {
		wg.Add(1)
		go func() {
			defer wg.Done()
			title := pack.RawTitle
			log.Printf("[%d/%d] VegaPulling images for: %s (%s)...\n", i, len(packs), title, pack.ID)
			outputDir := filepath.Join(vegaData, "images", pack.ID)

			cmdArgs := []string{"--language", language, "images", "--output-dir=" + outputDir, pack.ID, "-vv"}
			if err := runCommand(vegaBin, cmdArgs, ""); err != nil {
				errChan <- fmt.Errorf("failed to pull images for pack %s: %v", pack.ID, err)
				return
			}
			log.Printf("[%d/%d] Successfully VegaPulled images for: %s (%s) ✅\n", i, len(packs), title, pack.ID)
			// zip the output directory for easier git storage
			zipPath := filepath.Join(vegaData, "images", pack.ID+".zip")
			zipCmd := exec.Command("zip", "-r", zipPath, outputDir)
			if err := zipCmd.Run(); err != nil {
				errChan <- fmt.Errorf("failed to zip images for pack %s: %v", pack.ID, err)
				return
			}
			log.Printf("[%d/%d] Successfully zipped images for: %s (%s) to %s\n", i, len(packs), title, pack.ID, zipPath)
			err := os.RemoveAll(outputDir) // Clean up the directory after zipping
			if err != nil {
				errChan <- fmt.Errorf("failed to remove directory %s after zipping: %v", outputDir, err)
				return
			}
		}()
	}
	wg.Wait()
	close(errChan)
	for err := range errChan {
		if err != nil {
			return err
		}
	}
	return nil
}

// runCommand executes a command and writes output to a file if outFile is provided.
func runCommand(cmdPath string, args []string, outFile string) error {
	cmd := exec.Command(cmdPath, args...)
	// If outFile is provided, redirect stdout to the file.
	if outFile != "" {
		out, err := os.Create(outFile)
		if err != nil {
			return err
		}
		defer out.Close()
		cmd.Stdout = out
	} else {
		cmd.Stdout = os.Stdout
	}
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// exists checks if a file or directory exists.
func exists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}
