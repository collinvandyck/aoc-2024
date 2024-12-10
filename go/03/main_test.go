package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestEx1(t *testing.T) {
	val := eval(EX1, true)
	require.Equal(t, 161, val)
}

func TestEx2(t *testing.T) {
	val := eval(EX2, false)
	require.Equal(t, 48, val)
}

func TestPt1(t *testing.T) {
	val := eval(IN1, true)
	require.Equal(t, 185797128, val)
}

func TestPt2(t *testing.T) {
	val := eval(IN1, false)
	require.Equal(t, 89798695, val)
}
