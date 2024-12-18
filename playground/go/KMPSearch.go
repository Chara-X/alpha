package main

func KMPSearch(text, pattern string) []int {
	var pi = prefixFunction(pattern + "#" + text)
	var matches = []int{}
	for i := len(pattern) + 1; i < len(pi); i++ {
		if pi[i] == len(pattern) {
			matches = append(matches, i-2*len(pattern))
		}
	}
	return matches
}
func prefixFunction(str string) []int {
	var pi = make([]int, len(str))
	for i := 1; i < len(str); i++ {
		var len = pi[i-1]
		for len > 0 && str[len] != str[i] {
			len = pi[len-1]
		}
		if str[len] == str[i] {
			len++
		}
		pi[i] = len
	}
	return pi
}
