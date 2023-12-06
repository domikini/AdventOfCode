package main

import (
	"fmt"
)

func main() {
	// Create a dictionary map with a speed/timing key that takes a function
	dictionaryMap := make(map[Combo]int)

	for y := 0; y < 72; y++ {
		for i := 0; i < 72; i++ {
			combo := Combo{timing: i, speed: y}
			dictionaryMap[combo] = calculateRange(y, i)
		}
	}

	var race = Race{time: 72, distance: 1110}
	for x := 0; x < race.time; x++ {
		combo := Combo{speed: x, timing: race.time - x, distance: dictionaryMap[Combo{speed: x, timing: race.time - x}]}
		if combo.distance > race.distance {
			race.numberOfCombosPossibleToBeatDistance += 1
		}
		race.combos = append(race.combos, combo)
	}

	fmt.Println(race.numberOfCombosPossibleToBeatDistance)
}

type Race struct {
	combos                               []Combo
	time                                 int
	distance                             int
	numberOfCombosPossibleToBeatDistance int
}

type Combo struct {
	speed    int
	timing   int
	distance int
}

func calculateRange(speed int, timing int) int {
	return speed * timing
}
