package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

type Point struct {
	x int
	y int
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	risks0 := [][]uint{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		row := make([]uint, len(line)*5)
		for i, c := range line {
			n, _ := strconv.Atoi(string(c))
			for j := 0; j < 5; j++ {
				k := uint(n + j)
				if k >= 10 {
					k -= 9
				}
				row[(len(line)*j)+i] = k
			}
		}
		risks0 = append(risks0, row)
	}

	risks := make([][]uint, len(risks0)*5)
	lowests := make([][]uint, len(risks0)*5)
	for i := range lowests {
		lowests[i] = make([]uint, len(risks0)*5)
		for j := range lowests[i] {
			lowests[i][j] = math.MaxUint
		}
	}

	for i, row := range risks0 {
		for j := 0; j < 5; j++ {
			for _, n := range row {
				r := (len(risks0) * j) + i
				k := n + uint(j)
				if k >= 10 {
					k -= 9
				}
				risks[r] = append(risks[r], k)
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	lowests[0][0] = 0
	queue := []Point{{0, 0}}
	for len(queue) > 0 {
		x, y, i := findMin(queue, lowests)
		cost := lowests[y][x]
		queue = append(queue[:i], queue[i+1:]...)
		if x > 0 && cost+risks[y][x-1] < lowests[y][x-1] { // left
			lowests[y][x-1] = cost + risks[y][x-1]
			queue = append(queue, Point{x - 1, y})
		}
		if x < len(risks[0])-1 && cost+risks[y][x+1] < lowests[y][x+1] { // right
			lowests[y][x+1] = cost + risks[y][x+1]
			queue = append(queue, Point{x + 1, y})
		}
		if y > 0 && cost+risks[y-1][x] < lowests[y-1][x] { // top
			lowests[y-1][x] = cost + risks[y-1][x]
			queue = append(queue, Point{x, y - 1})
		}
		if y < len(risks)-1 && cost+risks[y+1][x] < lowests[y+1][x] { // bottom
			lowests[y+1][x] = cost + risks[y+1][x]
			queue = append(queue, Point{x, y + 1})
		}
		queue = uniqQueue(queue)
	}
	fmt.Println(lowests[len(risks)-1][len(risks[0])-1])
}

func findMin(queue []Point, lowests [][]uint) (int, int, int) {
	min := queue[0]
	index := 0

	for i, q := range queue {
		if lowests[q.y][q.x] < lowests[min.y][min.x] {
			min = q
			index = i
		}
	}

	return min.x, min.y, index
}

func uniqQueue(queue []Point) []Point {
	uniq := []Point{}

	for _, q := range queue {
		isUniq := true
		for _, u := range uniq {
			if q.x == u.x && q.y == u.y {
				isUniq = false
				break
			}
		}
		if isUniq {
			uniq = append(uniq, q)
		}
	}

	return uniq
}
