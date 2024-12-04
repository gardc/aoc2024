import Foundation

// Function to check if a report is safe based on Part One criteria
func isSafe(report: [Int]) -> Bool {
    // A report with less than 2 levels is trivially safe
    if report.count < 2 {
        return true
    }

    // Calculate the differences between adjacent levels
    var diffs: [Int] = []
    for i in 0..<(report.count - 1) {
        let diff = report[i + 1] - report[i]
        diffs.append(diff)
    }

    // Determine the direction of the first difference
    let firstDiff = diffs[0]
    if firstDiff == 0 {
        return false  // No change is not allowed
    }
    let isIncreasing = firstDiff > 0

    // Check all differences
    for diff in diffs {
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
        let absDiff = abs(diff)
        if absDiff < 1 || absDiff > 3 {
            return false
        }
    }

    return true
}

// Function to check if a report can be made safe by removing exactly one level
func isSafeWithOneRemoval(report: [Int]) -> Bool {
    // If the report is already safe, no need to remove anything
    if isSafe(report: report) {
        return true
    }

    // Try removing each level one by one and check if the resulting report is safe
    for i in 0..<report.count {
        var modifiedReport = report
        modifiedReport.remove(at: i)
        if isSafe(report: modifiedReport) {
            return true
        }
    }

    // If no single removal makes the report safe, return false
    return false
}

// Function to read input from a file and return an array of reports
func readInput(fromFile fileName: String) -> [[Int]] {
    // Print the current working directory for debugging
    let currentDirectory = FileManager.default.currentDirectoryPath
    print("Current Directory: \(currentDirectory)")

    // Get the file URL relative to the current directory
    let fileURL = URL(fileURLWithPath: currentDirectory)
        .appendingPathComponent(fileName)

    do {
        // Read the entire content of the file
        let content = try String(contentsOf: fileURL, encoding: .utf8)
        // Split the content into lines and then into integer arrays
        let reports =
            content
            .split(separator: "\n")
            .map { line in
                line
                    .split(separator: " ")
                    .compactMap { Int($0) }
            }
        return reports
    } catch {
        print("Error reading file: \(error)")
        return []
    }
}

// Main execution
func main() {
    // Use the relative path "../day2.txt"
    let reports = readInput(fromFile: "../day2.txt")

    var safeCountPart2 = 0

    for report in reports {
        if isSafeWithOneRemoval(report: report) {
            safeCountPart2 += 1
        }
    }

    print("Part Two: Number of safe reports with Problem Dampener = \(safeCountPart2)")
}

// Run the main function
main()
