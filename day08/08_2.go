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

	num := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		d := [14]string{}
		fmt.Sscanf(line, "%s %s %s %s %s %s %s %s %s %s | %s %s %s %s", &d[0], &d[1], &d[2], &d[3], &d[4], &d[5], &d[6], &d[7], &d[8], &d[9], &d[10], &d[11], &d[12], &d[13])

		dict := [10]string{}
		segBD := []rune{}
		segCF := []rune{}

		for _, s := range d {
			if len(s) == 2 { // 1
				dict[1] = s
				if dict[4] != "" {
					segBD = identifyBD(dict[1], dict[4])
				}
				segCF = []rune(s)
			} else if len(s) == 3 { // 7
				dict[7] = s
			} else if len(s) == 4 { // 4
				dict[4] = s
				if dict[1] != "" {
					segBD = identifyBD(dict[1], dict[4])
				}
			} else if len(s) == 7 { // 8
				dict[8] = s
			}
		}

		for _, s := range d {
			if len(s) == 5 { // 2, 3, 5
				if strings.IndexRune(s, segBD[0]) != -1 && strings.IndexRune(s, segBD[1]) != -1 {
					dict[5] = s
				} else if strings.IndexRune(s, segCF[0]) != -1 && strings.IndexRune(s, segCF[1]) != -1 {
					dict[3] = s
				} else {
					dict[2] = s
				}
			} else if len(s) == 6 { // 0, 6, 9
				if strings.IndexRune(s, segBD[0]) == -1 || strings.IndexRune(s, segBD[1]) == -1 {
					dict[0] = s
				} else if strings.IndexRune(s, segCF[0]) == -1 || strings.IndexRune(s, segCF[1]) == -1 {
					dict[6] = s
				} else {
					dict[9] = s
				}
			}
		}

		outputs := ""
		for _, digit := range d[10:] {
			for i, s := range dict {
				if isMatch(digit, s) {
					outputs += strconv.Itoa(i)
					break
				}
			}
		}
		n, _ := strconv.Atoi(outputs)
		num += n
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(num)
}

func identifyBD(d1, d4 string) []rune {
	bd := []rune{}

	for _, c := range d4 {
		i := strings.IndexRune(d1, c)
		if i == -1 {
			bd = append(bd, c)
		}
	}

	return bd
}

func isMatch(a, b string) bool {
	for _, c := range a {
		if strings.IndexRune(b, c) == -1 {
			return false
		}
	}
	for _, c := range b {
		if strings.IndexRune(a, c) == -1 {
			return false
		}
	}
	return true
}
