package main

import (
	"bufio"
	"fmt"
	"os"
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
	for fileScanner.Scan() {
		line := fileScanner.Text()
		var first, last int
		var f_present, l_present bool

		for _, c := range line {
			if unicode.IsDigit(c) {
				digit := int(c - '0')
				if !f_present {
					first = digit
					f_present = true
				} else {
					last = digit
					l_present = true
				}
			}
		}
		if !l_present {
			last = first
		}

		result += first*10 + last
	}
	fmt.Println(result)
}
