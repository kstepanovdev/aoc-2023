package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	file, err := os.Open("../input.txt")
	check(err)
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)

	var result int
	hmap := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}

	for fileScanner.Scan() {
		line := []rune(fileScanner.Text())
		var first, last int

		for i := 0; i < len(line); i++ {
			subres := Search(hmap, line[i:])

			if subres == 0 {
				continue
			}

			if first == 0 {
				first = subres
			} else {
				last = subres
			}
		}

		if first == 0 {
			continue
		} else {
			if last == 0 {
				result += first*10 + first
			} else {
				result += first*10 + last
			}
		}
	}
	fmt.Println(result)
}

func Search(hmap map[string]int, substr []rune) int {
	for _, r := range substr {
		if unicode.IsDigit(r) {
			return int(r - '0')
		} else {
			break
		}
	}

	for k, v := range hmap {
		if strings.HasPrefix(string(substr), k) {
			return v
		}
	}

	return 0
}
