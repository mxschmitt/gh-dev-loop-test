package main

import "testing"

func TestGreet(t *testing.T) {
    if got := greet("merlin"); got != "hello, merlin!" {
        t.Fatalf("got %q", got)
    }
}
