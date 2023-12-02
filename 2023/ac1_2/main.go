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
	re := regexp.MustCompile(`(oneight|twone|sevenine|eighthree|threeight|nineight|fiveight|eightwo|one|two|three|four|five|six|seven|eight|nine)`)

	var contentSplitFiltered []string
	for _, row := range contentSplit {
		matches := re.FindAllString(row, -1)
		for range matches {
			row = re.ReplaceAllStringFunc(row, func(match string) string {
				return convertToNumber(match)
			})
		}
		result := strings.Map(filterNumericChars, row)
		result = keepFirstAndLast(result)
		contentSplitFiltered = append(contentSplitFiltered, result)
	}
	var sum int64
	for _, row := range contentSplitFiltered {
		fmt.Println(row)
		i, err := strconv.ParseInt(row, 10, 64)
		if err != nil {
			// Handle the error (e.g., invalid string format)
			fmt.Println("Error:", err)
			return
		} else {
			fmt.Println("sum before: " + strconv.FormatInt(sum, 10))
			fmt.Println("i: " + strconv.FormatInt(i, 10))
			sum = sum + i
			fmt.Println("sum after: " + strconv.FormatInt(sum, 10))
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

func keepFirstAndLast(input string) string {
	if len(input) == 2 {
		return input
	} else if len(input) == 1 {
		return string(input[0]) + string(input[0])
	}
	// Return the first and last characters
	return string(input[0]) + string(input[len(input)-1])
}

func convertToNumber(s string) string {
	switch s {
	case "one":
		return "1"
	case "two":
		return "2"
	case "three":
		return "3"
	case "four":
		return "4"
	case "five":
		return "5"
	case "six":
		return "6"
	case "seven":
		return "7"
	case "eight":
		return "8"
	case "nine":
		return "9"
	case "oneight":
		return "18"
	case "twone":
		return "21"
	case "sevenine":
		return "79"
	case "eighthree":
		return "83"
	case "threeight":
		return "38"
	case "nineight":
		return "98"
	case "fiveight":
		return "58"
	case "eightwo":
		return "82"
	default:
		return s
	}
}
