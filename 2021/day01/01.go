package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	increased := 0
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	prev, _ := strconv.Atoi(scanner.Text())
	for scanner.Scan() {
		cur, _ := strconv.Atoi(scanner.Text())
		if cur > prev {
			increased += 1
		}
		prev = cur
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(increased)
}
