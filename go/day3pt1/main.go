package main

import (
	"fmt"
	"os"
	"regexp"
	"strings"
)

func parseStringToInstructionStrings(s string) []string {
	re := regexp.MustCompile(`mul\(\d+,\d+\)`)
	return re.FindAllString(s, -1)
}

func executeMul(s string) int {
	re := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	matches := re.FindStringSubmatch(s)
	if len(matches) != 3 {
		panic(fmt.Sprintf("could not dissect string in executeMul for %s", s))
	}

	var numbers []int
	for _, numStr := range matches[1:] {
		var num int
		fmt.Sscanf(numStr, "%d", &num)
		numbers = append(numbers, num)
	}

	res := numbers[0]
	for _, i := range numbers[1:] {
		res *= i
	}

	return res
}

func readFile(filename string) string {
	bytes, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	s := string(bytes)
	s = strings.ReplaceAll(s, "\n", "") // remove the \n
	return s
}

func main() {
	raw := readFile("../../day3.txt")
	instructionStrings := parseStringToInstructionStrings(raw)

	result := 0
	for _, is := range instructionStrings {
		result += executeMul(is)
	}

	fmt.Println("Result:", result)
}
