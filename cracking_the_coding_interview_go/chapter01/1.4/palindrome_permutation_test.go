package palindromepermutation

import (
	"testing"
)

func TestIsPalindromePermutation(t *testing.T) {
	inputString := "Tact Coa"

	if isPalindromePermutation(inputString) != true {
		t.Errorf("Should be true")
	}
}
