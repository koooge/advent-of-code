package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	horizontal := 0
	depth := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		l := strings.Split(scanner.Text(), " ")
		n, _ := strconv.Atoi(l[1])
		if l[0] == "forward" {
			horizontal += n
		} else if l[0] == "down" {
			depth = depth + n
		} else if l[0] == "up" {
			depth = depth - n
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(horizontal * depth)
}
