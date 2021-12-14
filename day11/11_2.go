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

	l := 0
	octs := [10][10]int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		row := [10]int{}
		fmt.Sscanf(line, "%1d%1d%1d%1d%1d%1d%1d%1d%1d%1d", &row[0], &row[1], &row[2], &row[3], &row[4], &row[5], &row[6], &row[7], &row[8], &row[9])

		octs[l] = row
		l++
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	num := 0
	for t := 0; t < 1000; t++ { // REVIEW: hardcode limit
		for i := 0; i < 10; i++ {
			for j := 0; j < 10; j++ {
				octs = gain(octs, i, j)
			}
		}

		all := true
		for i := 0; i < 10; i++ {
			for j := 0; j < 10; j++ {
				if octs[i][j] < 10 {
					all = false
				} else {
					octs[i][j] = 0
				}
			}
		}

		if all {
			num = t + 1
			break
		}
	}

	fmt.Println(num)
}

func gain(octs [10][10]int, i, j int) [10][10]int {
	if i < 0 || i > 9 || j < 0 || j > 9 {
		return octs
	}

	octs[i][j]++
	if octs[i][j] == 10 {
		octs = gain(octs, i-1, j-1) // top left
		octs = gain(octs, i-1, j)   // top
		octs = gain(octs, i-1, j+1) // top right
		octs = gain(octs, i, j-1)   // left
		octs = gain(octs, i, j+1)   // right
		octs = gain(octs, i+1, j-1) // bottom left
		octs = gain(octs, i+1, j)   // bottom
		octs = gain(octs, i+1, j+1) // bottom right
	}

	return octs
}
