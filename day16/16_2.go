package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	bins := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		bin := ""
		for _, hex := range line {
			n, err := strconv.ParseUint(string(hex), 16, 64)
			if err != nil {
				log.Fatal(err)
			}
			b := strconv.FormatUint(n, 2)
			for len(b) < 4 {
				b = "0" + b
			}
			bin += b
		}
		bins = append(bins, bin)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	for i := range bins {
		nums, _ := decode(bins[i])
		fmt.Println(nums[0])
	}
}

func decode(bins string) ([]uint64, string) {
	if bins == "" {
		return []uint64{}, ""
	}

	typeId := bins[3:6]
	if typeId == "100" {
		n, rest := decodeType4(bins[6:])
		return []uint64{n}, rest
	}

	nums := []uint64{}
	lenTypeId := bins[6]
	rest := ""
	if lenTypeId == '0' {
		length, err := strconv.ParseUint(bins[7:7+15], 2, 64)
		if err != nil {
			log.Fatal(err)
		}

		rest = bins[7+15:]
		for len(bins)-(7+15)-len(rest) < int(length) {
			n, tail := decode(rest)
			rest = tail
			nums = append(nums, n...)
		}
	} else if lenTypeId == '1' {
		length, err := strconv.ParseUint(bins[7:7+11], 2, 64)
		if err != nil {
			log.Fatal(err)
		}

		rest = bins[7+11:]
		for i := 0; i < int(length); i++ {
			n, tail := decode(rest)
			rest = tail
			nums = append(nums, n...)
		}
	}

	if typeId == "000" {
		var sum uint64 = 0
		for i := range nums {
			sum += nums[i]
		}
		return []uint64{sum}, rest
	} else if typeId == "001" {
		var sum uint64 = 1
		for i := range nums {
			sum *= nums[i]
		}
		return []uint64{sum}, rest
	} else if typeId == "010" {
		var min uint64 = math.MaxUint64
		for i := range nums {
			if nums[i] < min {
				min = nums[i]
			}
		}
		return []uint64{min}, rest
	} else if typeId == "011" {
		var max uint64 = 0
		for i := range nums {
			if nums[i] > max {
				max = nums[i]
			}
		}
		return []uint64{max}, rest
	} else if typeId == "101" {
		if nums[0] > nums[1] {
			return []uint64{1}, rest
		} else {
			return []uint64{0}, rest
		}
	} else if typeId == "110" {
		if nums[0] < nums[1] {
			return []uint64{1}, rest
		} else {
			return []uint64{0}, rest
		}
	} else if typeId == "111" {
		if nums[0] == nums[1] {
			return []uint64{1}, rest
		} else {
			return []uint64{0}, rest
		}
	} else {
		log.Fatal("Unknown typeId=", typeId)
	}

	return nums, rest
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
