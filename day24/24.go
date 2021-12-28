package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Monad struct {
	ins []string
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	monad := &Monad{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		monad.ins = append(monad.ins, line)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	var modelno string

	for i := 99999999999999; i > 11111111111111; i-- {
		no := strconv.Itoa(i)
		if strings.Contains(no, "0") {
			continue
		}

		if valid := monad.check(no); valid {
			modelno = no
			break
		}
	}

	fmt.Println(modelno)
}

func (m *Monad) check(modelno string) bool {
	store := make(map[string]uint64)

	for index := 0; index < len(modelno); {
		for _, line := range m.ins {
			s := strings.Split(line, " ")
			op := s[0]
			rega := s[1]

			if op == "inp" {
				n, _ := strconv.Atoi(string(modelno[index]))
				store[rega] = uint64(n)
				index++
				continue
			}

			regb := s[2]
			n := uint64(0)

			if regb == "w" || regb == "x" || regb == "y" || regb == "z" {
				n = store[regb]
			} else {
				i, _ := strconv.Atoi(string(regb))
				n = uint64(i)
			}

			if op == "add" {
				store[rega] += n
			} else if op == "mul" {
				store[rega] *= n
			} else if op == "div" {
				store[rega] /= n
			} else if op == "mod" {
				store[rega] %= n
			} else if op == "eql" {
				store[rega] = 0
				if store[rega] == n {
					store[rega] = 1
				}
			} else {
				panic("Unknown opcode")
			}
		}
	}

	return store["z"] == 0
}
