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
		if src != "end" && dst != "start" {
			route[src] = append(route[src], dst)
		}
		if dst != "end" && src != "start" {
			route[dst] = append(route[dst], src)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	paths := findPath(route, "start", "start", false)
	fmt.Println(len(paths))
}

func findPath(route map[string][]string, path string, pos string, twice bool) []string {
	paths := []string{}

	for _, next := range route[pos] {
		if next == "end" {
			paths = append(paths, path+","+next)
		} else if unicode.IsLower(rune(next[0])) {
			small := 0
			for _, p := range strings.Split(path, ",") {
				if next == p {
					small++
				}
			}
			if small < 1 {
				paths = append(paths, findPath(route, path+","+next, next, twice)...)
			} else if !twice && small == 1 {
				paths = append(paths, findPath(route, path+","+next, next, true)...)
			}
		} else {
			paths = append(paths, findPath(route, path+","+next, next, twice)...)
		}
	}

	return paths
}
