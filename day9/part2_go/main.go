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
		line := fileScanner.Text()
		var subres []int
		for _, c := range strings.Split(line, " ") {
			num, err := strconv.Atoi(string(c))
			check(err)
			subres = append(subres, num)
		}
		result += process(subres)

	}
	fmt.Println(result)
}

func process(input []int) int {
	var subarr [][]int
	subarr = append(subarr, input)

	curr_subarr := 0
	for !isFinished(subarr[curr_subarr]) {
		var buffer []int
		for i := 0; i < len(subarr[curr_subarr])-1; i++ {
			buffer = append(buffer, subarr[curr_subarr][i+1]-subarr[curr_subarr][i])
		}
		subarr = append(subarr, buffer)
		curr_subarr += 1
	}

	prev := 0
	for i := len(subarr) - 1; i >= 0; i-- {
		prev = subarr[i][0] - prev
	}

	return prev
}

func isFinished(input []int) bool {
	for i := 0; i < len(input); i++ {
		if input[i] != 0 {
			return false
		}
	}
	return true
}
