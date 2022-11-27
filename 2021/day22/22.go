package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Cube struct {
	turn [101][101][101]bool
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	procs := []string{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		procs = append(procs, line)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	cube := &Cube{}
	for _, proc := range procs {
		var x1, x2, y1, y2, z1, z2 int
		if strings.HasPrefix(proc, "on") {
			fmt.Sscanf(proc, "on x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2)
			cube.toggle(true, x1, x2, y1, y2, z1, z2)
		} else {
			fmt.Sscanf(proc, "off x=%d..%d,y=%d..%d,z=%d..%d", &x1, &x2, &y1, &y2, &z1, &z2)
			cube.toggle(false, x1, x2, y1, y2, z1, z2)
		}
	}

	fmt.Println(cube.count())
}

func (cube *Cube) toggle(onoff bool, x1, x2, y1, y2, z1, z2 int) {
	for i := x1; i <= x2; i++ {
		if i+50 < 0 || i+50 > 100 {
			continue
		}
		for j := y1; j <= y2; j++ {
			if j+50 < 0 || j+50 > 100 {
				continue
			}
			for k := z1; k <= z2; k++ {
				if k+50 < 0 || k+50 > 100 {
					continue
				}
				cube.turn[i+50][j+50][k+50] = onoff
			}
		}
	}
}

func (cube *Cube) count() int {
	num := 0
	for i := 0; i <= 100; i++ {
		for j := 0; j <= 100; j++ {
			for k := 0; k <= 100; k++ {
				if cube.turn[i][j][k] {
					num++
				}
			}
		}
	}
	return num
}
