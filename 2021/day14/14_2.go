package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	template := scanner.Text()
	rules := make(map[string]string)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		r := strings.Split(line, " -> ")
		rules[r[0]] = r[1]
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	polymers := make(map[string]int)
	for i, _ := range template[:len(template)-1] {
		k := string(template[i]) + string(template[i+1])
		polymers[k]++
	}

	elements := make(map[rune]int)
	for _, c := range template {
		elements[c]++
	}

	for i := 0; i < 40; i++ {
		tmp := make(map[string]int)
		for k, v := range polymers {
			left := string(k[0]) + rules[k]
			right := rules[k] + string(k[1])
			tmp[left] += v
			tmp[right] += v
			elements[[]rune(rules[k])[0]] += v
		}
		polymers = tmp
	}

	max := 0
	min := math.MaxInt
	for _, v := range elements {
		if v > max {
			max = v
		}
		if v < min {
			min = v
		}
	}
	fmt.Println(max - min)
}
