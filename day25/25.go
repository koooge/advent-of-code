package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Sea struct {
	area []string
	step int
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	sea := &Sea{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		sea.area = append(sea.area, line)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for sea.next() {
	}

	fmt.Println(sea.step)
}

func (s *Sea) next() bool {
	leny := len(s.area)
	lenx := len(s.area[0])
	isMoved := false
	s.step++

	// >
	for i := 0; i < leny; i++ {
		tmp1 := s.area[i][0]
		tmp2 := s.area[i][lenx-1]

		for j := 0; j < lenx-1; j++ {
			if s.area[i][j] == '>' && s.area[i][j+1] == '.' {
				s.area[i] = s.area[i][:j] + ".>" + s.area[i][j+2:]
				j++
				isMoved = true
			}
		}

		if tmp1 == '.' && tmp2 == '>' {
			s.area[i] = ">" + s.area[i][1:lenx-1] + "."
			isMoved = true
		}
	}

	// v
	for j := 0; j < lenx; j++ {
		tmp1 := s.area[0][j]
		tmp2 := s.area[leny-1][j]

		for i := 0; i < leny-1; i++ {
			if s.area[i][j] == 'v' && s.area[i+1][j] == '.' {
				s.area[i] = s.area[i][:j] + "." + s.area[i][j+1:]
				s.area[i+1] = s.area[i+1][:j] + "v" + s.area[i+1][j+1:]
				i++
				isMoved = true
			}
		}

		if tmp1 == '.' && tmp2 == 'v' {
			s.area[0] = s.area[0][:j] + "v" + s.area[0][j+1:]
			s.area[leny-1] = s.area[leny-1][:j] + "." + s.area[leny-1][j+1:]
			isMoved = true
		}
	}

	return isMoved
}
