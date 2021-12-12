package main

import (
	"bufio"
	"fmt"
	"log"
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

	nums := []int{}

	for i, row := range hmap {
		for j, _ := range row {
			if hmap[i][j] != 9 {
				n := getBasins(hmap, i, j)
				nums = append(nums, n)
			}
		}
	}
	sort.Ints(nums)

	fmt.Println(nums[len(nums)-3] * nums[len(nums)-2] * nums[len(nums)-1]) // 988624
}

func getBasins(hmap [][]int, i int, j int) int {
	n := 1
	hmap[i][j] = 9

	if j > 0 && hmap[i][j-1] != 9 {
		n += getBasins(hmap, i, j-1) // left
	}
	if j < len(hmap[i])-1 && hmap[i][j+1] != 9 {
		n += getBasins(hmap, i, j+1) // right
	}
	if i > 0 && hmap[i-1][j] != 9 {
		n += getBasins(hmap, i-1, j) // top
	}
	if i < len(hmap)-1 && hmap[i+1][j] != 9 {
		n += getBasins(hmap, i+1, j) // bottom
	}

	return n
}
