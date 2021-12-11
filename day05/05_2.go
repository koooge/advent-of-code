package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	fields := [1024][1024]int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var x1, y1, x2, y2 int
		fmt.Sscanf(line, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2)

		if x1 != x2 && y1 != y2 {
			fields[y1][x1] += 1
			for x1 != x2 && y1 != y2 {
				if x1 < x2 {
					x1++
				} else {
					x1--
				}
				if y1 < y2 {
					y1++
				} else {
					y1--
				}
				fields[y1][x1] += 1
			}
		} else if x1 != x2 {
			if x1 <= x2 {
				for i := x1; i <= x2; i++ {
					fields[y1][i] += 1
				}
			} else {
				for i := x2; i <= x1; i++ {
					fields[y1][i] += 1
				}
			}
		} else {
			if y1 <= y2 {
				for i := y1; i <= y2; i++ {
					fields[i][x1] += 1
				}
			} else {
				for i := y2; i <= y1; i++ {
					fields[i][x1] += 1
				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	num := 0
	for _, row := range fields {
		for _, n := range row {
			if n >= 2 {
				num++
			}
		}
	}
	fmt.Println(num)
}
