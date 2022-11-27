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
		outputs := strings.Split(strings.Split(line, "| ")[1], " ")
		for _, s := range outputs {
			if len(s) == 2 || len(s) == 3 || len(s) == 4 || len(s) == 7 {
				num++
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(num)
}
