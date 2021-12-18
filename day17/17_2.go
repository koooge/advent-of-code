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

	num := 0
	for x := 1; x <= highX; x++ {
		for y := lowY; float64(y) <= math.Abs(float64(lowY)); y++ {
			if isSuccess(x, y, lowX, highX, lowY, highY) {
				num++
			}
		}
	}

	fmt.Println(num)
}

func isSuccess(x, y, lowX, highX, lowY, highY int) bool {
	posX := 0
	posY := 0

	for t := 0; posX < highX && posY > lowY; t++ {
		if x-t > 0 {
			posX += x - t
		}
		posY += y - t

		if posX >= lowX && posX <= highX && posY >= lowY && posY <= highY {
			return true
		}
	}

	return false
}
