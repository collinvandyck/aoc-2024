package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestEx1(t *testing.T) {
	val := eval(EX1, true)
	require.Equal(t, 11, val)
}

func TestEx2(t *testing.T) {
	val := eval(EX1, false)
	require.Equal(t, 31, val)
}

func TestPt1(t *testing.T) {
	val := eval(IN1, true)
	require.Equal(t, val, 3569916)
}

func TestPt2(t *testing.T) {
	val := eval(IN1, false)
	require.Equal(t, val, 26407426)
}
