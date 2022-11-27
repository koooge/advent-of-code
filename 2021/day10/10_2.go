package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	nums := []int{}

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
					stack = []string{}
					break
				}
			} else if c == "]" {
				if stack[len(stack)-1] == "[" {
					stack = stack[:len(stack)-1]
				} else {
					stack = []string{}
					break
				}
			} else if c == "}" {
				if stack[len(stack)-1] == "{" {
					stack = stack[:len(stack)-1]
				} else {
					stack = []string{}
					break
				}
			} else if c == ">" {
				if stack[len(stack)-1] == "<" {
					stack = stack[:len(stack)-1]
				} else {
					stack = []string{}
					break
				}
			} else {
				stack = append(stack, c)
			}
		}

		n := 0
		for len(stack) > 0 {
			n *= 5
			if stack[len(stack)-1] == "(" {
				n += 1
			} else if stack[len(stack)-1] == "[" {
				n += 2
			} else if stack[len(stack)-1] == "{" {
				n += 3
			} else if stack[len(stack)-1] == "<" {
				n += 4
			} else {
				log.Fatal("Unknown value")
			}
			stack = stack[:len(stack)-1]
		}
		if n > 0 {
			nums = append(nums, n)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	sort.Ints(nums)
	fmt.Println(nums[len(nums)/2])
}
