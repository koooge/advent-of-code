package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	num := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		row := strings.Split(line, "")

		stack := []string{}
		for _, c := range row {
			if c == ")" {
				if stack[len(stack)-1] == "(" {
					stack = stack[:len(stack)-1]
				} else {
					num += 3
					break
				}
			} else if c == "]" {
				if stack[len(stack)-1] == "[" {
					stack = stack[:len(stack)-1]
				} else {
					num += 57
					break
				}
			} else if c == "}" {
				if stack[len(stack)-1] == "{" {
					stack = stack[:len(stack)-1]
				} else {
					num += 1197
					break
				}
			} else if c == ">" {
				if stack[len(stack)-1] == "<" {
					stack = stack[:len(stack)-1]
				} else {
					num += 25137
					break
				}
			} else {
				stack = append(stack, c)
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(num)
}
