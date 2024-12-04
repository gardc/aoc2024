package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// isSafe checks if a report is safe based on Part One criteria
func isSafe(report []int) bool {
	if len(report) < 2 {
		return true
	}

	diffs := make([]int, len(report)-1)
	for i := 0; i < len(report)-1; i++ {
		diffs[i] = report[i+1] - report[i]
	}

	// Determine the direction based on the first difference
	firstDiff := diffs[0]
	if firstDiff == 0 {
		return false
	}
	isIncreasing := firstDiff > 0

	for _, diff := range diffs {
		// Check for zero difference
		if diff == 0 {
			return false
		}

		// Check if all differences have the same direction
		if isIncreasing && diff <= 0 {
			return false
		}
		if !isIncreasing && diff >= 0 {
			return false
		}

		// Check if the absolute difference is between 1 and 3
		absDiff := diff
		if absDiff < 0 {
			absDiff = -absDiff
		}
		if absDiff < 1 || absDiff > 3 {
			return false
		}
	}

	return true
}

// isSafeWithOneRemoval checks if a report can be made safe by removing exactly one level
func isSafeWithOneRemoval(report []int) bool {
	if isSafe(report) {
		return true
	}

	for i := 0; i < len(report); i++ {
		modifiedReport := append([]int{}, report[:i]...)
		modifiedReport = append(modifiedReport, report[i+1:]...)
		if isSafe(modifiedReport) {
			return true
		}
	}

	return false
}

// readInput reads the input file and returns a slice of reports
func readInput(filename string) ([][]int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var reports [][]int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.TrimSpace(line) == "" {
			continue // Skip empty lines
		}
		parts := strings.Fields(line)
		var report []int
		for _, part := range parts {
			num, err := strconv.Atoi(part)
			if err != nil {
				// Skip non-integer values
				continue
			}
			report = append(report, num)
		}
		if len(report) > 0 {
			reports = append(reports, report)
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return reports, nil
}

func main() {
	reports, err := readInput("../../day2.txt")
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		return
	}

	safeCountPart2 := 0

	for _, report := range reports {
		if isSafeWithOneRemoval(report) {
			safeCountPart2++
		}
	}

	fmt.Printf("Part Two: Number of safe reports with Problem Dampener = %d\n", safeCountPart2)
}
