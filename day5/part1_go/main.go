package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

type Mapping struct {
	minCurrentLoc int
	minNextLoc    int
	mapping_size  int
}

func (m *Mapping) ContainsSeed(last_seed_loc int) bool {
	maxCurrentLoc := m.minCurrentLoc + m.mapping_size
	return last_seed_loc >= m.minCurrentLoc && last_seed_loc < maxCurrentLoc
}

func (m *Mapping) GetNextLoc(last_seed_loc int) int {
	return m.minNextLoc + absDiff(last_seed_loc, m.minCurrentLoc)
}

func absDiff(a, b int) int {
	if a > b {
		return a - b
	} else {
		return b - a
	}
}

func main() {
	file, err := os.Open("../input.txt")
	check(err)

	fileScanner := bufio.NewScanner(file)
	fileScanner.Split(bufio.ScanLines)
	seedLoc := make(map[int]int)
	var seeds []int
	var currMapping []Mapping
	seedsCollected := false
	junk := true

	for fileScanner.Scan() {
		line := fileScanner.Text()

		if strings.HasPrefix(line, "seeds:") {
			line := strings.Split(strings.TrimSpace(strings.Split(line, ":")[1]), " ")

			for _, num_cand := range line {
				num, err := strconv.Atoi(num_cand)
				check(err)
				seeds = append(seeds, num)
				seedLoc[num] = num
			}
		} else if len(line) == 0 {
			if !seedsCollected {
				seedsCollected = true
				continue
			}

			PopulateNextSeeds(&seeds, &seedLoc, &currMapping)
			currMapping = []Mapping{}
			junk = true
		} else {
			if junk {
				junk = false
				continue
			}

			// collect data
			line := strings.Split(line, " ")
			mapData, err := ConvertToInts(line)
			check(err)

			mapping := Mapping{}
			mapping.minNextLoc = mapData[0]
			mapping.minCurrentLoc = mapData[1]
			mapping.mapping_size = mapData[2]
			currMapping = append(currMapping, mapping)
		}
	}
	PopulateNextSeeds(&seeds, &seedLoc, &currMapping)

	min := math.MaxInt

	for _, value := range seedLoc {
		if value < min {
			min = value
		}
	}

	fmt.Println(min)
}

func ConvertToInts(source []string) ([]int, error) {
	var dest []int
	for _, numCandidate := range source {
		num, err := strconv.Atoi(numCandidate)
		if err != nil {
			return dest, err
		}
		dest = append(dest, num)
	}
	return dest, nil
}

func PopulateNextSeeds(seeds *[]int, seedLoc *map[int]int, currMapping *[]Mapping) {
	for _, seed := range *seeds {
		lastSeedLoc := (*seedLoc)[seed]

		for _, mapping := range *currMapping {
			if mapping.ContainsSeed(lastSeedLoc) {
				(*seedLoc)[seed] = mapping.GetNextLoc(lastSeedLoc)
			}
		}
	}
}
