package urlify


func ReplaceSpaces(str string, trueLength int) string {
	runes := []rune(str)
	spaceCount := 0
	i := 0

	for i = 0; i < trueLength; i++ {
		if runes[i] == ' ' {
			spaceCount++
		}
	}
	index := trueLength + spaceCount*2

	if trueLength < len(runes) {
		runes[trueLength] = '\000'
	}

	for i = trueLength - 1; i >= 0; i-- {
		if runes[i] == ' ' {
			runes[index-1] = '0'
			runes[index-2] = '2'
			runes[index-3] = '%'
			index = index - 3
		} else {
			runes[index-1] = runes[i]
			index--
		}
	}
	return string(runes)
}
