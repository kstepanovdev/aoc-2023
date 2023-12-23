package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
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
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " ")
		input := line[0]
		pattern := atoi(strings.Split(line[1], ","))

		result += patternScanner(input, pattern)

	}
	fmt.Println(result)
}

func patternScanner(input string, pattern []int) int {
	if len(input) == 0 {
		if len(pattern) == 0 {
			return 1
		} else {
			return 0
		}
	}

	if len(pattern) == 0 {
		if !strings.Contains(input, "#") {
			return 1
		} else {
			return 0
		}
	}

	curr_char := input[0]

	switch curr_char {
	case '.':
		return patternScanner(input[1:], pattern)
	case '?':
		sharp_input := replaceFirst(input, '#')
		dot_input := replaceFirst(input, '.')

		return patternScanner(sharp_input, pattern) + patternScanner(dot_input, pattern)
	case '#':
		curr_pattern := pattern[0]

		if len(input) >= curr_pattern && !anyDots(input[:curr_pattern]) && (len(input) == curr_pattern || input[curr_pattern] != '#') {
			if len(input) > curr_pattern {
				return patternScanner(input[curr_pattern+1:], pattern[1:])
			} else {
				return patternScanner(input[curr_pattern:], pattern[1:])
			}
		}
	}

	return 0
}

func replaceFirst(input string, replacement rune) string {
	buf := []rune(input)
	buf[0] = rune(replacement)
	return string(buf)
}

func anyDots(input string) bool {
	for _, c := range input {
		c := string(c)
		if c == "." {
			return true
		}
	}

	return false
}

func atoi(input []string) []int {
	var result []int

	for _, c := range input {
		c, err := strconv.Atoi(string(c))
		check(err)

		result = append(result, c)
	}

	return result
}
