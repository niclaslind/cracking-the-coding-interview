package palindromepermutation

import (
	"strings"
)

// Write a function to check if it is a permutation of a palindrome.
// A palindrome is a word or phrase that is the same forwards and backwards.
// A permutation is a rearrangement of letters

// Hash table
// Solve in O(n)
func isPalindromePermutation(s string) bool {
	s = strings.ToLower(s)

	//var result []string
	permutations := map[rune]int8{}

	for _, r := range s {
		permutations[r]++
	}

	return true
}

// Bit vector
// Solve in O(n) and reduced space
func isPalindromePermutationBitVector(s string) bool {
	//countOdd := 0
	//table := []int{'z' - 'a' + 1}
	//
	//for _, c := range s {
	//	x :=
	//}
	return true
}

// Map each character to a number. a -> 0, b -> 1, c -> 2 etc.
func getCharNumber(r rune) int {
	c := rune('c')
	a := rune('a')
	if a <= c && c <= 'z' {
		return int(c - a)
	}

	return -1
}

func buildCharFrequencyTable(phrase string) []int {

	z := rune('z')
	a := rune('a')

	table := make([]int, z-a+1)

	for _, c := range phrase {
		x := getCharNumber(c)

		if x != -1 {
			table[x]++
		}
	}

	return table
}
