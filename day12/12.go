package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	route := make(map[string][]string)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		src := strings.Split(line, "-")[0]
		dst := strings.Split(line, "-")[1]
		route[src] = append(route[src], dst)
		route[dst] = append(route[dst], src)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	paths := findPath(route, []string{"start"}, "start")

	fmt.Println(len(paths))
}

func findPath(route map[string][]string, path []string, pos string) [][]string {
	paths := [][]string{}

	for _, s := range route[pos] {
		if s == "start" {
			continue
		} else if s == "end" {
			paths = append(paths, append(path, s))
		} else {
			if unicode.IsLower(rune(s[0])) {
				smallAgain := false
				for _, p := range path {
					if s == p {
						smallAgain = true
					}
				}
				if smallAgain {
					continue
				}
			}
			paths = append(paths, findPath(route, append(path, s), s)...)
		}
	}

	return paths
}
