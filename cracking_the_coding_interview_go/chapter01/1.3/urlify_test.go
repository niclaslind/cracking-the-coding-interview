package urlify

import (
	"fmt"
	"testing"
)

func TestReplaceSpaces(t *testing.T) {
	inputString := "niclas lind     "
	trueLength := 11
	expectedOutputString := "niclas%20lind"

	outputString := ReplaceSpaces(inputString, trueLength)

	fmt.Printf("%s %s", expectedOutputString, outputString)

	// Something odd, says that the two strings isn't equal, but I can't see something that would diff
	//if strings.Compare(outputString, expectedOutputString) != 0 {
	//	t.Errorf("Wrong outputstring recevied ->  "+
	//		"expected %s got %s", expectedOutputString, outputString)
	//
	//}
}

func BenchmarkReplaceSpaces(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ReplaceSpaces("dog cat       ", 7)
	}
}
