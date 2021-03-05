package palindrome_permutation

import (
	"fmt"
	"testing"
)

func TestIsPalindromePermutation(t *testing.T) {
	inputString := "Tact Coa"
	//expectedOuput := []string{"taco cat", "atco cta"}

	fmt.Println(IsPalindromePermutation(inputString))
}
