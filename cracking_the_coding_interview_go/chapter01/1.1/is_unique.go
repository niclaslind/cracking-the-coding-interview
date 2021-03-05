package is_unique

// Takes a &str and check that all chars is unique
//
// Time complexity O(n) and Space complexity O(1)
// The time complexity can from one side also be O(1) because of that we only check strings
// that is smaller then 128
//
// Return true if that the case otherwise returns false
func isUniqueChars(s string) bool {
	if len(s) > 128 {
		return false
	}
	checker := make(map[rune]bool)
	for _, r := range s {
		if checker[r] == false {
			checker[r] = true
		} else {
			return false
		}
	}
	return true
}

// Takes a &str and check that all chars is unique
//
// Time complexity O(n) and Space complexity O(1)
// Here we also reduce the space complexity with eight from the previous solution
// by using a bit-vector instead of hashset
//
// In this case we assume that word is using only lowercase letters a..z
func isUniqueCharsBit(str string) bool {
	checker := 0
	for _, r := range str {
		val := r - 'a'
		if (checker & (1 << val)) > 0 {
			return false
		}
		checker |= 1 << val
	}
	return true
}
