package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestEx1(t *testing.T) {
	val := eval(EX1, true)
	require.Equal(t, 2, val)
}

func TestPt1(t *testing.T) {
	val := eval(IN1, true)
	require.Equal(t, val, 639)
}

func TestEx2(t *testing.T) {
	val := eval(EX1, false)
	require.Equal(t, 4, val)
}

func TestPt2(t *testing.T) {
	val := eval(IN1, false)
	require.Equal(t, val, 674)
}
