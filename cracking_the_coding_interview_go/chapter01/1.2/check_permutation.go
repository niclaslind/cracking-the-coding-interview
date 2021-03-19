package checkpermutation

import (
	"sort"
)

// Helper function for sorting string
func sortString(s string) string {
	r := []rune(s)
	sort.Slice(r, func(i, j int) bool {
		return r[i] < r[j]
	})
	return string(r)
}

// CheckPermutation -
// First of all, find out these things: Should the comparison be case sensitive?
// Example is God a permutation of dog?
//
// Second: Is whitespaces significant?
// Example "god      " is different from "dog"
func CheckPermutation(word, permutation string) bool {
	// check so the length matches
	if len(word) != len(permutation) {
		return false
	}

	return sortString(word) == sortString(permutation)
}

// CheckPermutationEfficient -
// This function is better if efficient is important, maybe not so readable as the first one
// regarding assumption, always check about the size of the character set, here we assume it is ASCII
func CheckPermutationEfficient(word, permutation string) bool {
	if len(word) != len(permutation) {
		return false
	}

	letters := new([128]int) // Assumption ASCII letters
	runes := []rune(word)

	// count number of each rune in word
	for _, r := range runes {
		letters[r]++
	}

	for i := 0; i < len(permutation); i++ {
		c := permutation[i]
		letters[c]--
		if letters[c] < 0 {
			return false
		}
	}

	return true
}
