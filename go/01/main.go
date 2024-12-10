package main

import (
	_ "embed"
	"fmt"
	"sort"
	"strings"
)

//go:embed data/ex1
var EX1 string

//go:embed data/in1
var IN1 string

func main() {}

func eval(s string, pt1 bool) int {
	l1, l2 := parse(s)
	sort.Ints(l1)
	sort.Ints(l2)
	var sum = 0
	for i := range l1 {
		if pt1 {
			n1, n2 := l1[i], l2[i]
			sum += abs(n1 - n2)
		} else {
			var count = 0
			for j := range l2 {
				if l2[j] == l1[i] {
					count += 1
				}
			}
			sum += l1[i] * count
		}
	}
	return sum
}

func abs(v int) int {
	if v < 0 {
		v *= -1
	}
	return v
}

func parse(s string) ([]int, []int) {
	s = strings.TrimSpace(s)
	lines := strings.Split(s, "\n")
	var l1, l2 []int
	for _, line := range lines {
		var x1, x2 int
		_, err := fmt.Sscanf(line, "%d %d", &x1, &x2)
		if err != nil {
			panic(err)
		}
		l1 = append(l1, x1)
		l2 = append(l2, x2)
	}
	return l1, l2
}
