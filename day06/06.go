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

	var state []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		st := strings.Split(line, ",")
		for _, c := range st {
			n, _ := strconv.Atoi(c)
			state = append(state, n)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < 80; i++ {
		for j, _ := range state {
			if state[j] > 0 {
				state[j]--
			} else {
				state[j] = 6
				state = append(state, 8)
			}
		}
	}

	fmt.Println(len(state))
}
