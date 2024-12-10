package main

import (
	_ "embed"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

//go:embed data/ex1
var EX1 string

//go:embed data/ex2
var EX2 string

//go:embed data/in1
var IN1 string

func eval(s string, pt1 bool) (res int) {
	var re1 = `mul\((\d+),(\d+)\)`
	var re *regexp.Regexp
	if pt1 {
		re = regexp.MustCompile(re1)
	} else {
		var re2 = `do\(\)|don't\(\)`
		re = regexp.MustCompile(fmt.Sprintf("%s|%s", re1, re2))
	}
	s = strings.TrimSpace(s)
	var enabled = true
	for _, match := range re.FindAllStringSubmatch(s, -1) {
		switch match[0] {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if !pt1 && !enabled {
				break
			}
			n1, err := strconv.Atoi(match[1])
			if err != nil {
				panic(err)
			}
			n2, err := strconv.Atoi(match[2])
			if err != nil {
				panic(err)
			}
			res += (n1 * n2)
		}
	}
	return
}
