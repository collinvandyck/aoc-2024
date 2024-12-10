package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed data/ex1
var EX1 string

//go:embed data/in1
var IN1 string

type report struct {
	levels []int
}

func eval(s string, pt1 bool) (res int) {
	reports := parse(s)
	var safe int
	for _, report := range reports {
		levels := report.levels
		all_levels := add_levels(levels, pt1)
		for _, levels := range all_levels {
			var inc, dec int
			var amt_ok = true
			for i := 1; i < len(levels); i++ {
				n1, n2 := levels[i-1], levels[i]
				var amt int
				if n2 > n1 {
					inc += 1
					amt = n2 - n1
				}
				if n2 < n1 {
					dec += 1
					amt = n1 - n2
				}
				if amt < 1 || amt > 3 {
					amt_ok = false
				}
			}
			if ((inc == 0 && dec != 0) || (inc != 0 && dec == 0)) && amt_ok {
				safe += 1
				break
			}
		}
	}
	return safe
}

func add_levels(levels []int, pt1 bool) (res [][]int) {
	res = append(res, levels)
	if !pt1 {
		for i := range levels {
			var sub []int
			sub = append(sub, levels[:i]...)
			sub = append(sub, levels[i+1:]...)
			res = append(res, sub)
		}
	}
	return

}

func parse(s string) (reports []report) {
	s = strings.TrimSpace(s)
	for _, line := range strings.Split(s, "\n") {
		reader := strings.NewReader(line)
		var nums []int
		for {
			var n int
			_, err := fmt.Fscan(reader, &n)
			if err != nil {
				break
			}
			nums = append(nums, n)
		}
		reports = append(reports, report{nums})
	}
	return
}
