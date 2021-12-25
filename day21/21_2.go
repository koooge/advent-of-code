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
	pos      int
	score    int
	universe uint64
}

func main() {
	file, err := os.Open("./example.txt")
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
		players = append(players, &Player{pos: n, universe: 1})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := 1; players[0].score < 21 && players[1].score < 21; i += 6 {
		players[0].add(i + i + 1 + i + 2)
		players[1].add(i + 3 + i + 4 + i + 5)
	}

	winuniverse := players[0].universe
	if players[1].score >= 21 {
		winuniverse = players[1].universe
	}

	fmt.Println(winuniverse)
}

func (player *Player) add(n int) {
	score := player.pos + n
	if score > 10 {
		score %= 10
		player.score += score
	}
	player.pos = score
	player.universe *= 3
}
