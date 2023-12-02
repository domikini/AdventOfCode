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
	var possibleGames []Game
	re := regexp.MustCompile(`(red|blue|green)`)
GameLoop:
	// Extract game
	for _, row := range contentSplit {
		isGamePossible := true
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
				if !checkIsGamePossible(comboObject) {
					// isGamePossible is set to true by default
					isGamePossible = false
					continue GameLoop
				}
				combos = append(combos, comboObject)
			}
			sets = append(sets, Set{combos: combos})
		}
		game := Game{id: strings.Map(filterNumericChars, rowSplit[0])}
		if isGamePossible {
			possibleGames = append(possibleGames, game)
		}
		games = append(games, game)
	}

	var sum int64
	for _, row := range possibleGames {
		i, err := strconv.ParseInt(row.id, 10, 64)
		if err != nil {
			// Handle the error (e.g., invalid string format)
			fmt.Println("Error:", err)
			return
		} else {
			sum = sum + i
		}
	}
	fmt.Println(sum)
}

func filterNumericChars(r rune) rune {
	if unicode.IsDigit(r) {
		return r
	}
	return -1
}

func checkIsGamePossible(combo Combo) bool {
	switch combo.color {
	case "red":
		if combo.number > 12 {
			return false
		}
		return true
	case "green":
		if combo.number > 13 {
			return false
		}
		return true
	case "blue":
		if combo.number > 14 {
			return false
		}
		return true
	default:
		return false
	}
}

type Game struct {
	id   string
	sets []Set
}

type Set struct {
	combos []Combo
}

type Combo struct {
	number int
	color  string
}
