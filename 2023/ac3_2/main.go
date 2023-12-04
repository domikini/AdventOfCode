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
	var combosEnriched []Combo
	for _, combo := range combos {
		combo = hasAdjacentGearSymbol(combo, schematic)
		combosEnriched = append(combosEnriched, combo)
	}

	// Filter combos with adjacentGearSymbol
	var comboWithAdjacentGearSymbol []Combo
	for _, combo := range combosEnriched {
		if combo.coordinateOfGearSymbol.y != 0 && combo.coordinateOfGearSymbol.x != 0 {
			comboWithAdjacentGearSymbol = append(comboWithAdjacentGearSymbol, combo)
		}
	}

	// Group part numbers to a map
	gearMap := make(map[Coordinate][]string)
	for _, combo := range comboWithAdjacentGearSymbol {
		gearMap[combo.coordinateOfGearSymbol] = append(gearMap[combo.coordinateOfGearSymbol], combo.line)
	}

	// Calculate gear ratio and total sum
	var sum int
	for _, value := range gearMap {
		if len(value) == 2 {
			value1, err := strconv.Atoi(value[0])
			if err != nil {
				fmt.Println("Error:", err)
				return
			}
			value2, err := strconv.Atoi(value[1])
			sum = sum + (value1 * value2)
		}
	}
	// Print the result
	fmt.Println(sum)
}

func hasAdjacentGearSymbol(combo Combo, schematic [][]rune) Combo {
	coordinates := combo.coordinates
	for _, coordinate := range coordinates {
		// left
		if coordinate.x > 0 {
			if isGearSymbol(schematic[coordinate.y][coordinate.x-1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y, x: coordinate.x - 1}
			}
		}
		// right
		if coordinate.x < len(schematic[0])-1 {
			if isGearSymbol(schematic[coordinate.y][coordinate.x+1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y, x: coordinate.x + 1}
			}
		}
		// upper left
		if coordinate.x > 0 && coordinate.y > 0 {
			if isGearSymbol(schematic[coordinate.y-1][coordinate.x-1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y - 1, x: coordinate.x - 1}
			}
		}
		// up
		if coordinate.y > 0 {
			if isGearSymbol(schematic[coordinate.y-1][coordinate.x]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y - 1, x: coordinate.x}
			}
		}
		// upper right
		if coordinate.x < len(schematic[0])-1 && coordinate.y > 0 {
			if isGearSymbol(schematic[coordinate.y-1][coordinate.x+1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y - 1, x: coordinate.x + 1}
			}
		}
		// down left
		if coordinate.x > 0 && coordinate.y < len(schematic)-1 {
			if isGearSymbol(schematic[coordinate.y+1][coordinate.x-1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y + 1, x: coordinate.x - 1}
			}
		}
		// down
		if coordinate.y < len(schematic)-1 {
			if isGearSymbol(schematic[coordinate.y+1][coordinate.x]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y + 1, x: coordinate.x}
			}
		}
		// down right
		if coordinate.x < len(schematic[0])-1 && coordinate.y < len(schematic)-1 {
			if isGearSymbol(schematic[coordinate.y+1][coordinate.x+1]) {
				combo.coordinateOfGearSymbol = Coordinate{y: coordinate.y + 1, x: coordinate.x + 1}
			}
		}
	}
	return combo
}

func isGearSymbol(symbol rune) bool {
	re := regexp.MustCompile(`(\*)`)
	matches := re.FindAllString(string(symbol), -1)
	if len(matches) > 0 {
		return true
	}
	return false
}

type Combo struct {
	line                   string
	coordinates            []Coordinate
	coordinateOfGearSymbol Coordinate
}

type Coordinate struct {
	x int
	y int
}
