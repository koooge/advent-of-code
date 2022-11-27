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

	hmap := [][]int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		row := []int{}
		rowstr := strings.Split(line, "")
		for _, c := range rowstr {
			n, _ := strconv.Atoi(c)
			row = append(row, n)
		}

		hmap = append(hmap, row)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	num := 0

	for i, row := range hmap {
		for j, _ := range row {
			if j == 0 || (j > 0 && hmap[i][j] < hmap[i][j-1]) { // left
				if i == 0 || i > 0 && hmap[i][j] < hmap[i-1][j] { // top
					if j == len(row)-1 || hmap[i][j] < hmap[i][j+1] { // right
						if i == len(hmap)-1 || hmap[i][j] < hmap[i+1][j] { // bottom
							num += hmap[i][j] + 1
						}
					}
				}
			}
		}
	}

	fmt.Println(num)
}
