package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type Coord struct {
	i int
	j int
}

func main() {
	file, err := os.Open("../input.txt")
	check(err)
	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	var coords []Coord
	var galaxyMap [][]string

	for fileScanner.Scan() {
		line := fileScanner.Text()
		var buf []string
		for _, c := range line {
			buf = append(buf, string(c))
		}
		galaxyMap = append(galaxyMap, buf)
	}

	paddingI := 0
	for i := 0; i < len(galaxyMap); i++ {
		allDots := true
		paddingJ := 0
		for j := 0; j < len(galaxyMap[i]); j++ {
			if galaxyMap[i][j] == "#" {
				coords = append(coords, Coord{i: i + paddingI, j: j + paddingJ})
				allDots = false
			} else {
				allDots := true
				for i := 0; i < len(galaxyMap); i++ {
					if galaxyMap[i][j] == "#" {
						allDots = false
						break
					}
				}
				if allDots {
					paddingJ += 1000000 - 1
				}
			}
		}

		if allDots {
			paddingI += 1000000 - 1
		}
	}

	result := 0
	for i, curr_galaxy := range coords {
		for next_galaxy := i + 1; next_galaxy < len(coords); next_galaxy++ {
			result += abs(coords[next_galaxy].i-curr_galaxy.i) + abs(coords[next_galaxy].j-curr_galaxy.j)
		}
	}

	fmt.Println(result)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
