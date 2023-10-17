package chinaid

import (
	"testing"
)

func TestParse(t *testing.T) {
	id, err := New("43102220200101133X")
	if err != nil {
		t.Fatal(err)
	}

	if id.Adcode() != "431022" {
		t.Fatal("wrong adcode")
	}

	if _, str := id.Birthday(); str != "20200101" {
		t.Fatal("wrong birthday")
	}

	if !id.Male() {
		t.Fatal("wrong gender")
	}
}

func BenchmarkParse(b *testing.B) {
	for i := 0; i < b.N; i++ {
		New("43102220200101133X")
	}
}
