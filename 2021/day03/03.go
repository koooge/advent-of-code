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

	ones := [32]int{}
	digits := 0
	lines := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines += 1
		line := scanner.Text()
		digits = len(line)

		for i, c := range line {
			if c == '1' {
				ones[i] += 1
			}
		}
	}

	gamma := ""
	epsilon := ""
	th := lines / 2
	for i := 0; i < digits; i++ {
		if ones[i] >= th {
			gamma += "1"
			epsilon += "0"
		} else {
			gamma += "0"
			epsilon += "1"
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	n, _ := strconv.ParseInt(gamma, 2, 64)
	e, _ := strconv.ParseInt(epsilon, 2, 64)
	fmt.Println(n * e)
}
