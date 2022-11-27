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

	scanner := bufio.NewScanner(file)

	scanner.Scan()
	head := strings.Split(scanner.Text(), ",")
	numbers := []int{}
	for _, c := range head {
		n, _ := strconv.Atoi(c)
		numbers = append(numbers, n)
	}
	cards := [][5][5]int{}

	for scanner.Scan() {
		card := [5][5]int{}
		for i := 0; i < 5; i++ {
			scanner.Scan()
			line := scanner.Text()
			fmt.Sscanf(line, "%2d %2d %2d %2d %2d", &card[i][0], &card[i][1], &card[i][2], &card[i][3], &card[i][4])
		}
		cards = append(cards, card)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	bingo := make([]bool, len(cards))
	rest := len(cards)
	for _, n := range numbers {
		for k := 0; k < len(cards); k++ {
			if bingo[k] {
				continue
			}

			for i := 0; i < 5; i++ {
				for j := 0; j < 5; j++ {
					if cards[k][i][j] == n {
						cards[k][i][j] = -1
						if isBingo(cards[k]) {
							bingo[k] = true
							if rest == 1 {
								unmarkedNum := sumUnmarked(cards[k])
								fmt.Println(unmarkedNum * n)
								return
							}
							rest--
						}
					}
				}
			}
		}
	}

	fmt.Println(cards)
	log.Fatal("no bingo")
}

func isBingo(card [5][5]int) bool {
	for i := 0; i < 5; i++ {
		if card[0][i] == -1 && card[1][i] == -1 && card[2][i] == -1 && card[3][i] == -1 && card[4][i] == -1 {
			return true
		}
		if card[i][0] == -1 && card[i][1] == -1 && card[i][2] == -1 && card[i][3] == -1 && card[i][4] == -1 {
			return true
		}
	}

	return false
}

func sumUnmarked(card [5][5]int) int {
	sum := 0
	for _, r := range card {
		for _, n := range r {
			if n == -1 {
				continue
			}
			sum += n
		}
	}
	return sum
}
