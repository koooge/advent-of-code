package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Node struct {
	Parent   *Node
	Left     *Node
	Right    *Node
	LeftNum  int
	RightNum int
}

func main() {
	file, err := os.Open("./example.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	list := [][][3]int{}
	index := -1

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		} else if !strings.HasPrefix(line, "---") {
			var n [3]int
			fmt.Sscanf(line, "%d,%d,%d", &n[0], &n[1], &n[2])
			list[index] = append(list[index], n)
		} else {
			fmt.Sscanf(line, "--- scanner %d ---", &index)
			list = append(list, [][3]int{})
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	vectors := [8][3]int{
		{1, 1, 1},
		{1, 1, -1},
		{1, -1, 1},
		{-1, 1, 1},
		{-1, -1, 1},
		{-1, 1, -1},
		{1, -1, -1},
		{-1, -1, -1},
	}

	becons := 0

	for _, p1 := range list[1:] {

	}

	fmt.Println(becons)
}

func matchNum(v1, v2 [][3]int) int {
	ret := 0

	for i := range v1 {
		for j := range v2 {
			if v1[i][0] == v2[j][0] && v1[i][1] == v2[j][1] && v1[i][2] == v2[j][2] {
				ret++
			}
		}
	}

	return ret
}
