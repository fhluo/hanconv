package main

import (
	"io"
	"log"
	"log/slog"
	"os"
	"path/filepath"
	"sync"

	"github.com/go-git/go-billy/v5"
	"github.com/go-git/go-billy/v5/memfs"
	"github.com/go-git/go-billy/v5/util"
	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
	"github.com/go-git/go-git/v5/storage/memory"
)

func init() {
	log.SetFlags(0)
}

func CopyFile(fs billy.Filesystem, src string, dst string) error {
	r, err := fs.Open(src)
	if err != nil {
		return err
	}
	defer func() {
		if err := r.Close(); err != nil {
			slog.Warn("failed to close file", "filename", src, "error", err)
		}
	}()

	w, err := os.Create(dst)
	if err != nil {
		return err
	}
	defer func() {
		if err := w.Close(); err != nil {
			slog.Warn("failed to close file", "filename", dst, "error", err)
		}
	}()

	_, err = io.Copy(w, r)
	return err
}

func main() {
	dirs := os.Args[1:]
	if len(dirs) == 0 {
		dirs = []string{"./data/dictionary"}
	}
	for _, dir := range dirs {
		if err := os.MkdirAll(dir, os.ModePerm); err != nil {
			slog.Warn("failed to create dir", "dir", dir, "error", err)
		}
	}

	fs := memfs.New()
	_, err := git.Clone(memory.NewStorage(), fs, &git.CloneOptions{
		URL:           "https://github.com/BYVoid/OpenCC",
		ReferenceName: plumbing.Master,
		SingleBranch:  true,
		Depth:         1,
		Progress:      os.Stdout,
	})
	if err != nil {
		slog.Error("failed to clone repo", "err", err)
		os.Exit(1)
	}

	files, err := util.Glob(fs, "data/dictionary/*.txt")
	if err != nil {
		panic(err)
	}

	var wg sync.WaitGroup
	for _, filename := range files {
		wg.Go(func() {
			for _, dir := range dirs {
				dst := filepath.Join(dir, filepath.Base(filename))
				if err := CopyFile(fs, filename, dst); err != nil {
					slog.Error("failed to copy file", "src", filename, "dst", dst, "err", err)
				}
			}
		})
	}
	wg.Wait()
}
