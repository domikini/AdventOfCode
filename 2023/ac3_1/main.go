package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	var schematic [][]rune
	content, err := os.ReadFile("input")
	if err != nil {
		log.Fatal(err)
	}
	// Read map into schematic - a 2D array
	contentSplit := strings.Split(string(content), "\n")
	for i := 0; i < len(contentSplit); i++ {
		line := contentSplit[i]
		row := []rune(line)
		schematic = append(schematic, row)
	}

	var line string
	var combo Combo
	var combos []Combo
	var coordinates []Coordinate
	// Find all combos
	re := regexp.MustCompile(`(\d)`)
	for y := 0; y < len(schematic); y++ {
		for x := 0; x < len(schematic[0]); x++ {
			match := re.FindAllString(string(schematic[y][x]), -1)
			if len(match) > 0 {
				coordinates = append(coordinates, Coordinate{y: y, x: x})
				line = line + match[0]
			} else {
				if len(line) > 0 {
					combo.line = line
					combo.coordinates = coordinates
					combos = append(combos, combo)
					line = ""
					coordinates = []Coordinate{}
				}
			}
		}
	}
	var isAdjacentCombos []Combo
	for _, combo := range combos {
		if isAdjacentNumber(combo, schematic) {
			isAdjacentCombos = append(isAdjacentCombos, combo)
		}
	}

	// Calculate sum
	var sum int
	for _, combo := range isAdjacentCombos {
		num, err := strconv.Atoi(combo.line)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		sum = sum + num
	}

	// Print the result
	fmt.Println(sum)
}

func isAdjacentNumber(combo Combo, schematic [][]rune) bool {
	var adjacentValues []rune
	coordinates := combo.coordinates
	for _, coordinate := range coordinates {
		// left
		if coordinate.x > 0 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y][coordinate.x-1])
		}
		// right
		if coordinate.x < len(schematic[0])-1 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y][coordinate.x+1])
		}
		// upper left
		if coordinate.x > 0 && coordinate.y > 0 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y-1][coordinate.x-1])
		}
		// up
		if coordinate.y > 0 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y-1][coordinate.x])
		}
		// upper right
		if coordinate.x < len(schematic[0])-1 && coordinate.y > 0 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y-1][coordinate.x+1])
		}
		// down left
		if coordinate.x > 0 && coordinate.y < len(schematic)-1 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y+1][coordinate.x-1])
		}
		// down
		if coordinate.y < len(schematic)-1 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y+1][coordinate.x])
		}
		// down right
		if coordinate.x < len(schematic[0])-1 && coordinate.y < len(schematic)-1 {
			adjacentValues = append(adjacentValues, schematic[coordinate.y+1][coordinate.x+1])
		}
	}
	adjacentValuesString := string(adjacentValues)
	re := regexp.MustCompile(`([^.|\d])`)
	matches := re.FindAllString(adjacentValuesString, -1)
	if len(matches) > 0 {
		return true
	}
	return false
}

type Combo struct {
	line        string
	coordinates []Coordinate
}

type Coordinate struct {
	x int
	y int
}
