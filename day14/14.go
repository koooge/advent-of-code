package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	template := scanner.Text()
	rules := make(map[string]string)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		r := strings.Split(line, " -> ")
		rules[r[0]] = r[1]
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 0; i < 10; i++ {
		tmp := ""
		tail := string(template[len(template)-1])
		for j := 0; j < len(template)-1; j++ {
			k := string(template[j]) + string(template[j+1])
			tmp += string(template[j]) + rules[k]
		}
		template = tmp + tail
	}

	max := 0
	min := len(template)
	counted := ""
	for _, c := range template {
		if strings.Index(counted, string(c)) != -1 {
			continue
		}
		counted += string(c)
		n := strings.Count(template, string(c))
		if n > max {
			max = n
		}
		if n < min {
			min = n
		}
	}
	fmt.Println(max - min)
}
