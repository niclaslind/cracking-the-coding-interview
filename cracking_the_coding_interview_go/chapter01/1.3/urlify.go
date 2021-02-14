package urlify

func Urlify(s string) string {
	var returnString string
	lastSpace := false

	for _, i := range s {
		if string(i) == " " && lastSpace == false {
			returnString += "%20"
			lastSpace = true
		} else if string(i) == " " && lastSpace == true {
			// do nothing on this one
		} else {
			returnString += string(i)
			lastSpace = false
		}
	}

	return returnString
}

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
