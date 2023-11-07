package chinaid

import (
	"errors"
	"strconv"
	"strings"
	"time"
)

const _LENGTH = 18

var _COEFFICIENT = []int64{7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2}
var _CHECK = []byte{'1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'}

var ErrLength = errors.New("id length must be 18")
var ErrInvalidDate = errors.New("invalid birthday format")
var ErrNotNumber = errors.New("non-string characters found in non-check bits")
var ErrInvalidId = errors.New("invalid id card number")

type ChinaId string

type Gender uint

const (
	Male Gender = iota
	Female
)

func New(str string) ChinaId {
	return ChinaId(strings.ToUpper(str))
}

func (id ChinaId) Valid() error {
	if len(id) != _LENGTH {
		return ErrLength
	}

	var sum int64

	for i, c := range id[:_LENGTH-1] {
		// index: 0-17
		// must be number
		num, err := strconv.ParseInt(string(c), 10, 64)
		if err != nil {
			return ErrNotNumber
		}

		sum += num * _COEFFICIENT[i]
	}

	// index: 18
	if _CHECK[sum%11] != id[_LENGTH-1] {
		return ErrInvalidId
	}

	if _, err := id.Birthday(); err != nil {
		return err
	}

	return nil
}

func (id ChinaId) Adcode() string {
	return string(id[0:6])
}

func (id ChinaId) Birthday() (time.Time, error) {
	if len(id) != _LENGTH {
		return time.Time{}, ErrLength
	}

	birthday, err := time.Parse("20060102", string(id[6:6+8]))
	if err != nil {
		return time.Time{}, ErrInvalidDate
	}
	return birthday, nil
}

func (id ChinaId) Gender() Gender {
	num, err := strconv.ParseInt(string(id[16]), 10, 64)
	if err == nil {
		if num%2 == 0 {
			return Female
		}
	}
	return Male
}
