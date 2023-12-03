package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func main() {

	content, err := os.ReadFile("input")
	if err != nil {
		log.Fatal(err)
	}
	contentSplit := strings.Split(string(content), "\n")
	var games []Game
	re := regexp.MustCompile(`(red|blue|green)`)
	// Extract game
	for _, row := range contentSplit {
		minimum := GameMinimum{}
		// Extract game id
		rowSplit := strings.Split(row, ": ")
		var sets []Set
		// Extract game sets
		gameSplit := strings.Split(rowSplit[1], "; ")
		for _, set := range gameSplit {
			var combos []Combo
			setSplit := strings.Split(set, ", ")
			for _, combo := range setSplit {
				// Convert rune to numeric value
				number, _ := strconv.Atoi(strings.Map(filterNumericChars, combo))
				comboObject := Combo{number: number, color: re.FindString(combo)}
				switch comboObject.color {
				case "red":
					if comboObject.number > minimum.Red {
						minimum.Red = comboObject.number
					}
					break
				case "blue":
					if comboObject.number > minimum.Blue {
						minimum.Blue = comboObject.number
					}
					break
				case "green":
					if comboObject.number > minimum.Green {
						minimum.Green = comboObject.number
					}
					break
				default:
				}
				combos = append(combos, comboObject)
			}
			sets = append(sets, Set{combos: combos})
		}
		power := minimum.Green * minimum.Blue * minimum.Red
		game := Game{id: strings.Map(filterNumericChars, rowSplit[0]), minimum: minimum, power: power}
		games = append(games, game)
	}

	var sum int
	for _, row := range games {
		sum = sum + row.power
	}
	fmt.Println(sum)
}

func filterNumericChars(r rune) rune {
	if unicode.IsDigit(r) {
		return r
	}
	return -1
}

type GameMinimum struct {
	Red   int
	Blue  int
	Green int
}

type Game struct {
	id      string
	sets    []Set
	minimum GameMinimum
	power   int
}

type Set struct {
	combos []Combo
}

type Combo struct {
	number int
	color  string
}
