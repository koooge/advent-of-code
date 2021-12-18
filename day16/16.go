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

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	bins := ""

	for _, hex := range line {
		n, err := strconv.ParseUint(string(hex), 16, 64)
		if err != nil {
			log.Fatal(err)
		}
		bin := strconv.FormatUint(n, 2)
		for len(bin) < 4 {
			bin = "0" + bin
		}
		bins += bin
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	sumVer, _ := decode(bins)
	fmt.Println(sumVer)
}

func decode(bins string) (uint64, string) {
	if bins == "" {
		return 0, ""
	}

	versionId := bins[0:3]
	ver, err := strconv.ParseUint(versionId, 2, 64)
	if err != nil {
		log.Fatal(err)
	}
	typeId := bins[3:6]

	if typeId == "100" {
		_, rest := decodeType4(bins[6:])
		return ver, rest
	}

	lenTypeId := bins[6]
	if lenTypeId == '0' {
		length, err := strconv.ParseUint(bins[7:7+15], 2, 64)
		if err != nil {
			log.Fatal(err)
		}

		rest := bins[7+15:]
		for len(bins)-(7+15)-len(rest) < int(length) {
			v, tail := decode(rest)
			rest = tail
			ver += v
		}
		return ver, rest
	} else if lenTypeId == '1' {
		length, err := strconv.ParseUint(bins[7:7+11], 2, 64)
		if err != nil {
			log.Fatal(err)
		}

		rest := bins[7+11:]
		for i := 0; i < int(length); i++ {
			v, tail := decode(rest)
			rest = tail
			ver += v
		}
		return ver, rest
	}
	return 0, ""
}

func decodeType4(bin string) (uint64, string) {
	i := 0
	packet := ""
	for bin[i] == '1' {
		packet += bin[i+1 : i+5]
		i += 5
	}
	packet += bin[i+1 : i+5]
	n, err := strconv.ParseUint(packet, 2, 64)
	if err != nil {
		log.Fatal(err)
	}

	return n, bin[i+5:]
}
