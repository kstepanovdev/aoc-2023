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

		input := strings.Repeat(line[0]+"?", 4) + line[0]
		pattern := atoi(strings.Split(strings.Repeat(line[1]+",", 4)+line[1], ","))

		var states []rune
		states = append(states, '.')

		for i := 0; i < len(pattern); i++ {
			for j := 0; j < pattern[i]; j++ {
				states = append(states, '#')
			}
			states = append(states, '.')
		}
		statesTransitions := make(map[int]int)
		statesTransitions[0] = 1

		result += calcTransitions(input, states, statesTransitions)

	}
	fmt.Println(result)
}

func calcTransitions(input string, states []rune, statesTransitions map[int]int) int {
	newTransitions := make(map[int]int)
	for _, char := range input {
		for state, val := range statesTransitions {
			switch char {
			case '?':
				if state+1 < len(states) {
					newTransitions[state+1] += val
				}
				if states[state] == '.' {
					newTransitions[state] += val
				}
			case '#':
				if state+1 < len(states) && states[state+1] == '#' {
					newTransitions[state+1] += val
				}
			case '.':
				if state+1 < len(states) && states[state+1] == '.' {
					newTransitions[state+1] += val
				}
				if states[state] == '.' {
					newTransitions[state] += val
				}
			}
		}
		statesTransitions = newTransitions
		newTransitions = make(map[int]int)
	}

	return statesTransitions[len(states)-1] + statesTransitions[len(states)-2]
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
