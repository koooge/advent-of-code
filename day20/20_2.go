package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Node struct {
	Parent   *Node
	Left     *Node
	Right    *Node
	LeftNum  int
	RightNum int
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	image := [][]int{}
	scanner.Scan()
	algorithm := scanner.Text()

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		row := []int{}
		for _, c := range line {
			if c == '.' {
				row = append(row, 0)
			} else {
				row = append(row, 1)
			}
		}
		image = append(image, row)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < 50; i++ {
		out := algorithm[0]
		if out == '#' && i%2 == 0 {
			out = algorithm[len(algorithm)-1]
		}
		if out == '#' {
			image = enhance(image, algorithm, 1)
		} else {
			image = enhance(image, algorithm, 0)
		}
	}

	lit := 0
	for i := range image {
		for j := range image[i] {
			if image[i][j] == 1 {
				lit++
			}
		}
	}

	fmt.Println(lit)
}

func enhance(in [][]int, algorithm string, outside int) [][]int {
	ret := make([][]int, len(in)+2)
	for i := range ret {
		ret[i] = make([]int, len(in[0])+2)
	}

	for i := -1; i < len(in)+1; i++ {
		for j := -1; j < len(in[0])+1; j++ {
			pos := [9]int{outside, outside, outside, outside, outside, outside, outside, outside, outside}
			if i >= 1 && j >= 1 {
				pos[0] = in[i-1][j-1]
			}
			if i >= 1 && j >= 0 && j <= len(in[0])-1 {
				pos[1] = in[i-1][j]
			}
			if i >= 1 && j <= len(in[0])-2 {
				pos[2] = in[i-1][j+1]
			}
			if i >= 0 && i <= len(in)-1 && j >= 1 {
				pos[3] = in[i][j-1]
			}
			if i >= 0 && i <= len(in)-1 && j >= 0 && j <= len(in[0])-1 {
				pos[4] = in[i][j]
			}
			if i >= 0 && i <= len(in)-1 && j <= len(in[0])-2 {
				pos[5] = in[i][j+1]
			}
			if i <= len(in)-2 && j >= 1 {
				pos[6] = in[i+1][j-1]
			}
			if i <= len(in)-2 && j >= 0 && j <= len(in[0])-1 {
				pos[7] = in[i+1][j]
			}
			if i <= len(in)-2 && j <= len(in[0])-2 {
				pos[8] = in[i+1][j+1]
			}

			pixels := fmt.Sprintf("%d%d%d%d%d%d%d%d%d", pos[0], pos[1], pos[2], pos[3], pos[4], pos[5], pos[6], pos[7], pos[8])
			d, _ := strconv.ParseInt(pixels, 2, 10)
			if algorithm[d] == '#' {
				ret[i+1][j+1] = 1
			} else {
				ret[i+1][j+1] = 0
			}
		}
	}

	return ret
}
