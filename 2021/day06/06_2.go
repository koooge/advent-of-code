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

	state := [7]int{}
	stateNew := [9]int{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		st := strings.Split(line, ",")
		for _, c := range st {
			n, _ := strconv.Atoi(c)
			state[n] += 1
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < 256; i++ {
		tmp := state[0] + stateNew[0]
		for j := 0; j < 6; j++ {
			state[j] = state[j+1]
			stateNew[j] = stateNew[j+1]
		}
		state[6] = tmp
		stateNew[6] = stateNew[7]
		stateNew[7] = stateNew[8]
		stateNew[8] = tmp
	}

	num := stateNew[7] + stateNew[8]
	for i := 0; i < 7; i++ {
		num += state[i] + stateNew[i]
	}

	fmt.Println(num)
}
