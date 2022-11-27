package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Player struct {
	pos    int
	score  int
	rolled int
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	players := []*Player{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		s := strings.Split(line, ": ")
		n, _ := strconv.Atoi(s[1])
		players = append(players, &Player{pos: n})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 1; ; i += 6 {
		players[0].add(i + i + 1 + i + 2)
		if players[0].score >= 1000 {
			break
		}
		players[1].add(i + 3 + i + 4 + i + 5)
		if players[1].score >= 1000 {
			break
		}
	}

	losescore := players[0].score
	if players[0].score >= 1000 {
		losescore = players[1].score
	}

	fmt.Println(losescore * (players[0].rolled + players[1].rolled))
}

func (player *Player) add(n int) {
	score := (player.pos + n) % 10
	if score == 0 {
		score = 10
	}
	player.pos = score
	player.score += score
	player.rolled += 3
}
