package is_unique

import "testing"

func TestIsUniqueChars(t *testing.T) {
	isUnique := isUniqueChars("Hi")
	if isUnique != true {
		t.Errorf("Should be unique")
	}

	isNotUnique := isUniqueChars("HiThere")
	if isNotUnique != false {
		t.Errorf("Should not be unique")
	}
}

func BenchmarkIsUniqueChars(b *testing.B) {
	for i := 0; i < b.N; i++ {
		isUniqueChars("Hej")
	}
}

func TestIsUniqueCharsBit(t *testing.T) {
	isUnique := isUniqueCharsBit("niclas")
	if isUnique != true {
		t.Errorf("Should be unique")
	}

	if isNotUnique := isUniqueCharsBit("niclaslind"); isNotUnique != false {
		t.Errorf("Should not be unique")
	}
}

func BenchmarkIsUniqueCharsBit(b *testing.B) {
	for i := 0; i < b.N; i++ {
		isUniqueCharsBit("niclas")
	}
}
