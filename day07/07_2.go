package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	pos := []int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		st := strings.Split(line, ",")
		for _, c := range st {
			n, _ := strconv.Atoi(c)
			pos = append(pos, n)
		}
	}
	sort.Ints(pos)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	cheap := 100000000 // REVIEW: hardcode
	for i := pos[0]; i < pos[len(pos)-1]; i++ {
		fuel := 0
		for j, _ := range pos {
			// REVIEW: skip dupe
			d := int(math.Abs(float64(i - pos[j])))
			fuel += ((d + 1) * d) / 2
		}
		if fuel < cheap {
			cheap = fuel
		}
	}

	fmt.Println(cheap)
}
