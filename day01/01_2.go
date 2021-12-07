package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var arr []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		n, _ := strconv.Atoi(scanner.Text())
		arr = append(arr, n)
	}

	increased := 0
	prev := arr[0] + arr[1] + arr[2]
	for i := 1; i < len(arr)-2; i += 1 {
		cur := arr[i] + arr[i+1] + arr[i+2]
		if cur > prev {
			increased += 1
		}
		prev = cur
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(increased)
}
