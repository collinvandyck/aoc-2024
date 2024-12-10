package main

import (
	_ "embed"
	"strings"
)

//go:embed data/ex1
var EX1 string

//go:embed data/in1
var IN1 string

type grid struct {
	chs [][]rune
}

func (g grid) find(pt1 bool) (count int) {
	for r, row := range g.chs {
		for c, ch := range row {
			if pt1 {
				if ch != 'X' {
					continue
				}
				checks := [][]rune{
					[]rune{g.get(r, c+1), g.get(r, c+2), g.get(r, c+3)},       // right
					[]rune{g.get(r, c-1), g.get(r, c-2), g.get(r, c-3)},       // left
					[]rune{g.get(r-1, c), g.get(r-2, c), g.get(r-3, c)},       // up
					[]rune{g.get(r+1, c), g.get(r+2, c), g.get(r+3, c)},       // down
					[]rune{g.get(r-1, c+1), g.get(r-2, c+2), g.get(r-3, c+3)}, // upright
					[]rune{g.get(r-1, c-1), g.get(r-2, c-2), g.get(r-3, c-3)}, // upleft
					[]rune{g.get(r+1, c+1), g.get(r+2, c+2), g.get(r+3, c+3)}, // downright
					[]rune{g.get(r+1, c-1), g.get(r+2, c-2), g.get(r+3, c-3)}, // downleft
				}
				for _, check := range checks {
					if string(check) == "MAS" {
						count += 1
					}
				}
			} else {
				if ch != 'A' {
					continue
				}
				var ul, ur, dl, dr = g.get(r-1, c-1), g.get(r-1, c+1), g.get(r+1, c-1), g.get(r+1, c+1)
				var s = string([]rune{ul, ur, dl, dr})
				for _, check := range []string{
					"MSMS",
					"SMSM",
					"SSMM",
					"MMSS",
				} {
					if s == check {
						count += 1
					}
				}
			}
		}
	}
	return
}

func (g grid) get(r, c int) rune {
	if r >= 0 && r < len(g.chs) {
		row := g.chs[r]
		if c >= 0 && c < len(row) {
			return g.chs[r][c]
		}
	}
	return '💀'
}

func eval(s string, pt1 bool) (res int) {
	grid := parse(s)
	return grid.find(pt1)
}

func parse(s string) (res grid) {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	for _, line := range lines {
		runes := []rune(line)
		res.chs = append(res.chs, runes)
	}
	return
}
