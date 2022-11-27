package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		data = append(data, line)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	digits := len(data[0])

	oxygenGenerator := append([]string{}, data...)
	for i := 0; i < digits; i++ {
		if len(oxygenGenerator) == 1 {
			break
		}

		ones := 0
		for j := 0; j < len(oxygenGenerator); j++ {
			if oxygenGenerator[j][i] == '1' {
				ones++
			}
		}

		remain := '0'
		if ones*2 >= len(oxygenGenerator) {
			remain = '1'
		}

		tmp := []string{}
		for _, s := range oxygenGenerator {
			if []rune(s)[i] == remain {
				tmp = append(tmp, s)
			}
		}
		oxygenGenerator = append([]string{}, tmp...)
	}

	co2Scrubber := append([]string{}, data...)
	for i := 0; i < digits; i++ {
		if len(co2Scrubber) == 1 {
			break
		}

		zeros := 0
		for j := 0; j < len(co2Scrubber); j++ {
			if co2Scrubber[j][i] == '0' {
				zeros++
			}
		}

		remain := '1'
		if zeros*2 <= len(co2Scrubber) {
			remain = '0'
		}

		tmp := []string{}
		for _, s := range co2Scrubber {
			if []rune(s)[i] == remain {
				tmp = append(tmp, s)
			}
		}
		co2Scrubber = append([]string{}, tmp...)
	}

	o2, _ := strconv.ParseInt(oxygenGenerator[0], 2, 64)
	co2, _ := strconv.ParseInt(co2Scrubber[0], 2, 64)
	fmt.Println(o2 * co2)
}
