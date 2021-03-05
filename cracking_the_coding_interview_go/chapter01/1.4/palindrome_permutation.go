package palindrome_permutation

import (
	"fmt"
	"strings"
)

// Write a function to check if it is a permutation of a palindrome.
// A palindrome is a word or phrase that is the same forwards and backwards.
// A permutation is a rearrangement of letters

// Hash table
// Solve in O(n)
func IsPalindromePermutation(s string) bool {
	s = strings.ToLower(s)

	//var result []string
	permutations := map[rune]int8{}

	for _, r := range s {
		permutations[r]++
	}

	fmt.Println(permutations)

	return true
}

// Bit vector
// Solve in O(n) and reduced space
func isPalindromePermutation(s string) bool {
	//countOdd := 0
	//table := []int{'z' - 'a' + 1}
	//
	//for _, c := range s {
	//	x :=
	//}
}
