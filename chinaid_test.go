package chinaid

import (
	"testing"
)

func TestParse(t *testing.T) {
	id := New("43102220200101133x")

	if err := id.Valid(); err != nil {
		t.Fatal(err)
	}

	if id.Adcode() != "431022" {
		t.Fatal("wrong adcode")
	}

	if date, err := id.Birthday(); err != nil || date.Format("20060102") != "20200101" {
		t.Fatal("wrong birthday")
	}

	if id.Gender() != Male {
		t.Fatal("wrong gender")
	}
}

func BenchmarkParse(b *testing.B) {
	for i := 0; i < b.N; i++ {
		id := New("43102220200101133x")
		id.Valid()
		id.Adcode()
		id.Birthday()
		id.Gender()
	}
}
