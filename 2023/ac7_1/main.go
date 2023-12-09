package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"slices"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// Read input file
	content, err := os.ReadFile("input")
	if err != nil {
		log.Fatal(err)
	}

	cardsWeightMap := make(map[string]int)
	cardsWeightMap["A"] = 13
	cardsWeightMap["K"] = 12
	cardsWeightMap["Q"] = 11
	cardsWeightMap["J"] = 10
	cardsWeightMap["T"] = 9
	cardsWeightMap["9"] = 8
	cardsWeightMap["8"] = 7
	cardsWeightMap["7"] = 6
	cardsWeightMap["6"] = 5
	cardsWeightMap["5"] = 4
	cardsWeightMap["4"] = 3
	cardsWeightMap["3"] = 2
	cardsWeightMap["2"] = 1

	var hands []Hand

	// Parse content
	contentSplit := strings.Split(string(content), "\n")
	for _, row := range contentSplit {
		rowSplit := strings.Split(row, " ")
		bid, _ := strconv.Atoi(rowSplit[1])
		hand := Hand{cards: rowSplit[0], bid: bid}
		hand.comboType = isComboType(hand.cards)
		hands = append(hands, hand)
	}

	sort.SliceStable(hands, func(i, j int) bool {
		if hands[i].comboType == hands[j].comboType {
			for x := 0; x < len(hands[i].cards); x++ {
				if hands[i].cards[x] == hands[j].cards[x] {
					continue
				} else {
					return cardsWeightMap[string(hands[i].cards[x])] > cardsWeightMap[string(hands[j].cards[x])]
				}
			}
		}
		return hands[i].comboType > hands[j].comboType
	})

	highestRanklevel := len(hands)
	var total int
	for y := 0; y < len(hands); y++ {
		result := hands[y].bid * (highestRanklevel - y)
		total += result
	}
	fmt.Println(total)
}

type Hand struct {
	cards     string
	bid       int
	comboType int
}

func isComboType(combo string) int {
	runeArray := []rune(combo)
	slices.Sort(runeArray)
	//Five
	regexpFive := regexp.MustCompile(`(A{5}|K{5}|Q{5}|J{5}|T{5}|9{5}|8{5}|7{5}|6{5}|5{5}|4{5}|3{5}|2{5})`)
	matchFive := regexpFive.FindAllString(string(runeArray), -1)
	//Four
	regexpFour := regexp.MustCompile(`(A{4}|K{4}|Q{4}|J{4}|T{4}|9{4}|8{4}|7{4}|6{4}|5{4}|4{4}|3{4}|2{4})`)
	matchFour := regexpFour.FindAllString(string(runeArray), -1)
	//Triple
	regexpTriple := regexp.MustCompile(`(A{3}|K{3}|Q{3}|J{3}|T{3}|9{3}|8{3}|7{3}|6{3}|5{3}|4{3}|3{3}|2{3})`)
	matchTriple := regexpTriple.FindAllString(string(runeArray), -1)
	//Pairs
	regexpPairs := regexp.MustCompile(`(A{2}|K{2}|Q{2}|J{2}|T{2}|9{2}|8{2}|7{2}|6{2}|5{2}|4{2}|3{2}|2{2})`)
	matchPairs := regexpPairs.FindAllString(string(runeArray), -1)

	if len(matchFive) == 1 {
		//Five of a kind
		return 500
	} else if len(matchFour) == 1 {
		//Four of a kind
		return 400
	} else if len(matchTriple) == 1 && len(matchPairs) == 2 {
		//Full house
		return 350
	} else if len(matchTriple) == 1 && len(matchPairs) == 1 {
		//Three of a kind
		return 300
	} else if len(matchPairs) == 2 {
		//Two pairs
		return 200
	} else if len(matchPairs) == 1 {
		//One pair
		return 100
	} else {
		//Distinct
		return 0
	}
}
