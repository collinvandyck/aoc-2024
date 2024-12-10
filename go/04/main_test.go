package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestEx1(t *testing.T) {
	val := eval(EX1, true)
	require.Equal(t, 18, val)
}

func TestPt1(t *testing.T) {
	val := eval(IN1, true)
	require.Equal(t, 2458, val)
}

func TestEx2(t *testing.T) {
	val := eval(EX1, false)
	require.Equal(t, 9, val)
}

func TestPt2(t *testing.T) {
	val := eval(IN1, false)
	require.Equal(t, 1945, val)
}
