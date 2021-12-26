package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Range struct {
	head int
	tail int
}

type Cube struct {
	x []Range
	y []Range
	z []Range
}

func main() {
	file, err := os.Open("./example.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	cube := &Cube{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var x, y, z Range
		if strings.HasPrefix(line, "on") {
			fmt.Sscanf(line, "on x=%d..%d,y=%d..%d,z=%d..%d", &x.head, &x.tail, &y.head, &y.tail, &z.head, &z.tail)
			cube.addRange(true, x, y, z)
		} else {
			fmt.Sscanf(line, "off x=%d..%d,y=%d..%d,z=%d..%d", &x.head, &x.tail, &y.head, &y.tail, &z.head, &z.tail)
			cube.addRange(false, x, y, z)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(cube)
	fmt.Println(cube.count())
}

func (cube *Cube) addRange(on bool, x, y, z Range) {
	cube.x = calcRange(cube.x, on, x)
	cube.y = calcRange(cube.y, on, y)
	cube.z = calcRange(cube.z, on, z)
}

func calcRange(ranges []Range, on bool, r Range) []Range {
	if on {
		if len(ranges) == 0 {
			return append(ranges, r)
		} else if r.head > ranges[len(ranges)-1].tail {
			return append([]Range{r}, ranges...)
		}

		for i := 0; i < len(ranges); i++ {
			if r.tail < ranges[i].head {
				ranges = append(ranges, r)
				break
			}

			if r.head < ranges[i].head && r.tail >= ranges[i].head && r.tail <= ranges[i].tail {
				ranges[i].head = r.head
			} else if r.head <= ranges[i].head && r.tail >= ranges[i].tail {
				ranges[i].head = r.head
				ranges[i].tail = r.tail // TODO: next Range
			} else if r.head >= ranges[i].head && r.tail >= ranges[i].head && r.tail >= ranges[i].tail {
				ranges[i].tail = r.tail
			}
		}
	} else { // off
		for i := 0; i < len(ranges); i++ {
			if r.head <= ranges[i].head && r.tail > ranges[i].head {
				ranges[i].head = r.tail + 1
			}
			if r.head < ranges[i].tail && r.tail >= ranges[i].tail {
				ranges[i].tail = r.head - 1
			}
		}
	}

	return ranges
}

func (cube *Cube) count() int {
	var nx, ny, nz int

	for _, r := range cube.x {
		nx += r.tail - r.head + 1
	}

	for _, r := range cube.y {
		ny += r.tail - r.head + 1
	}

	for _, r := range cube.z {
		nz += r.tail - r.head + 1
	}

	return nx * ny * nz
}
