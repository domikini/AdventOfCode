package main

import (
	"fmt"
	"log"
	"os"
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
	var contentSplitFiltered []string
	for _, row := range contentSplit {
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

func keepFirstAndLast(input string) string {
	if len(input) == 2 {
		// Return the original string if it has less than 2 characters
		return input
	} else if len(input) == 1 {
		return string(input[0]) + string(input[0])
	}
	// Return the first and last characters
	return string(input[0]) + string(input[len(input)-1])
}
