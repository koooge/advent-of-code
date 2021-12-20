package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./example.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	list := [][2]interface{}{}

	for scanner.Scan() {
		line := scanner.Text()

		list = append(list, parseList(line))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(list[0])
	result := [2]interface{}{}
	// for i := 0; i < len(list)-1; i++ {
	// 	result = reduce([2]interface{}{list[i], list[i+1]})
	// }
	result = reduce([2]interface{}{list[0], list[0+1]})

	fmt.Println(result)
}

func parseList(in string) [2]interface{} {
	arr := [2]interface{}{}
	if len(in) == 5 {
		a0, _ := strconv.Atoi(string(in[1]))
		a1, _ := strconv.Atoi(string(in[3]))
		arr = [2]interface{}{a0, a1}
	} else if in[1] == '[' {
		i := findCloseIndex(in[1:]) + 1
		arr[0] = parseList(in[1 : i+1])
		arr[1] = in[i+2 : len(in)-1]
	} else {
		a0, _ := strconv.Atoi(string(in[1]))
		arr[0] = a0
		arr[1] = parseList(in[3 : len(in)-1])
	}

	return arr
}

func findCloseIndex(in string) int {
	d := 0

	for i, _ := range in {
		if in[i] == ']' {
			d--
			if d == 0 {
				return i
			}
		} else if in[i] == '[' {
			d++
		}
	}

	return 0
}

func reduce(in [2]interface{}) [2]interface{} {
	res := [2]interface{}{}

	fmt.Println(in)
	fmt.Println(len(in))

	// explode
	for i := range in {
		a, ok := in[i].([2]interface{})
		if !ok {
			continue
		}
		for j := range a {
			b, ok := a[j].([2]interface{})
			if !ok {
				continue
			}
			for k := range b {
				c, ok := b[k].([2]interface{})
				if !ok {
					continue
				}
				for _, d := range c {
					fmt.Println(d)
				}
			}
		}
	}
	// explode

	// split

	return res
}
