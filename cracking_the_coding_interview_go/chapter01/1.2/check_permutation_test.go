package check_permutation

import "testing"

func TestCheckPermutation(t *testing.T) {
	isPermutation := CheckPermutation("hello", "olleh")
	if isPermutation != true {
		t.Errorf("Should be permutation")
	}

	isNotPermutation := CheckPermutation("hello", "wello")
	if isNotPermutation != false {
		t.Errorf("Should not be permutation")
	}
}

func BenchmarkCheckPermutation(b *testing.B) {
	for i := 0; i < b.N; i++ {
		CheckPermutation("hello", "elloh")
	}
}

func TestCheckPermutationEfficient(t *testing.T) {
	isPermutation := CheckPermutationEfficient("ascii", "sciia")
	if isPermutation != true {
		t.Errorf("Should be permutation")
	}

	isNotPermutation := CheckPermutationEfficient("ascii", "wasii")
	if isNotPermutation != false {
		t.Errorf("Should not be permutaion")
	}
}

func BenchmarkCheckPermutationEfficient(b *testing.B) {
	for i := 0; i < b.N; i++ {
		CheckPermutationEfficient("hello", "hello")
	}
}
