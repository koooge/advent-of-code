package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var lowX, highX, lowY, highY int

	for scanner.Scan() {
		line := scanner.Text()
		fmt.Sscanf(line, "target area: x=%d..%d, y=%d..%d", &lowX, &highX, &lowY, &highY)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	maxY := findHighest(lowY, highY)
	fmt.Println(maxY)
}

func findHighest(lowY, highY int) int {
	max := 0

	for y := 1; float64(y) < math.Abs(float64(lowY)); y++ {
		posY := 0
		maxY := 0
		for t := 0; posY > lowY; t++ {
			posY += y - t
			if posY > maxY {
				maxY = posY
			}
			if posY >= lowY && posY <= highY {
				if maxY > max {
					max = maxY
				}
				break
			}
		}
	}

	return max
}
