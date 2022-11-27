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

	origami := [1500][1500]bool{} // REVIEW: hardcode
	xlen := 0
	ylen := 0
	folds := []string{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		if !strings.HasPrefix(line, "fold") {
			var x, y int
			fmt.Sscanf(line, "%d,%d", &x, &y)
			origami[y][x] = true
			if x+1 > xlen {
				xlen = x + 1
			}
			if y+1 > ylen {
				ylen = y + 1
			}
		} else {
			folds = append(folds, strings.Split(line, " ")[2])
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for _, fold := range folds {
		d := strings.Split(fold, "=")[1]
		n, _ := strconv.Atoi(d)

		if strings.HasPrefix(fold, "y") {
			for i := n + 1; i < ylen; i++ {
				for j := 0; j < xlen; j++ {
					if origami[i][j] == true {
						d := i - n
						origami[n-d][j] = true
					}
				}
			}
			ylen = n
		} else if strings.HasPrefix(fold, "x") {
			for i := 0; i < ylen; i++ {
				for j := n + 1; j < xlen; j++ {
					if origami[i][j] == true {
						d := j - n
						origami[i][n-d] = true
					}
				}
			}
			xlen = n
		}
		break
	}

	num := 0
	for i := 0; i < ylen; i++ {
		for j := 0; j < xlen; j++ {
			if origami[i][j] {
				num++
			}
		}
	}

	fmt.Println(num)
}
