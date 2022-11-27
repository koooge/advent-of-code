package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Cube struct {
	x1, x2 int
	y1, y2 int
	z1, z2 int
	on     bool
	cubes  []*Cube
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	cubes := []*Cube{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		c := &Cube{}
		if strings.HasPrefix(line, "on") {
			fmt.Sscanf(line, "on x=%d..%d,y=%d..%d,z=%d..%d", &c.x1, &c.x2, &c.y1, &c.y2, &c.z1, &c.z2)
			c.on = true
		} else {
			fmt.Sscanf(line, "off x=%d..%d,y=%d..%d,z=%d..%d", &c.x1, &c.x2, &c.y1, &c.y2, &c.z1, &c.z2)
		}
		ret := []*Cube{}

		for _, cube := range cubes {
			ob, isOverlap := findOverlap(c, cube)
			if !isOverlap {
				continue
			}

			if c.on && ob.on {
				ob.on = false
			} else if !c.on && !ob.on {
				ob.on = true
			} else {
				ob.on = c.on
			}
			ret = append(ret, ob)
		}

		if c.on {
			cubes = append(cubes, c)
		}
		cubes = append(cubes, ret...)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	num := 0
	for _, cube := range cubes {
		if cube.on {
			num += cube.count()
		} else {
			num -= cube.count()
		}
	}

	fmt.Println(num)
}

func findOverlap(c1, c2 *Cube) (overlap *Cube, isOverlap bool) {
	newx1 := max(c1.x1, c2.x1)
	newx2 := min(c1.x2, c2.x2)
	if newx1 > newx2 {
		return
	}

	newy1 := max(c1.y1, c2.y1)
	newy2 := min(c1.y2, c2.y2)
	if newy1 > newy2 {
		return
	}

	newz1 := max(c1.z1, c2.z1)
	newz2 := min(c1.z2, c2.z2)
	if newz1 > newz2 {
		return
	}

	overlap = &Cube{on: c2.on, x1: newx1, x2: newx2, y1: newy1, y2: newy2, z1: newz1, z2: newz2}
	isOverlap = true
	return
}

func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}

func min(a, b int) int {
	if b < a {
		return b
	}
	return a
}

func (cube *Cube) count() int {
	return (cube.x2 - cube.x1 + 1) * (cube.y2 - cube.y1 + 1) * (cube.z2 - cube.z1 + 1)
}
