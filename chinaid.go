package chinaid

import (
	"errors"
	"strconv"
	"strings"
	"time"
)

const ID_LENGTH = 18

var _COEFFICIENT = []int64{7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2}
var _CHECK = []byte{'1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'}

var ErrLength = errors.New("id length must be 18")
var ErrInvalidDate = errors.New("invalid birthday date")
var ErrNotNumber = errors.New("non-string characters found in non-check bits")
var ErrInvalidId = errors.New("invalid id card number")

type ChinaId struct {
	raw      string
	birthday time.Time
	female   bool
}

func New(str string) (*ChinaId, error) {
	str = strings.ToUpper(str)

	if len(str) != 18 {
		return nil, ErrLength
	}

	var sum int64
	var female bool

	for i, c := range str[:ID_LENGTH-1] {
		// index: 0-17
		// must be number
		num, err := strconv.ParseInt(string(c), 10, 64)
		if err != nil {
			return nil, ErrNotNumber
		}

		sum += num * _COEFFICIENT[i]

		if i == ID_LENGTH-1 {
			// male or female
			female = num%2 == 0
		}
	}

	birthday, err := time.Parse("20060102", str[6:6+8])
	if err != nil {
		return nil, ErrInvalidDate
	}

	// index: 18
	if _CHECK[sum%11] != str[ID_LENGTH-1] {
		return nil, ErrInvalidId
	}

	//  ALL Good
	return &ChinaId{
		raw:      str,
		female:   female,
		birthday: birthday,
	}, nil
}

func Must(id *ChinaId, err error) *ChinaId {
	if err != nil {
		panic(err)
	}
	return id
}

func (id *ChinaId) Adcode() string {
	return id.raw[:6]
}

func (id *ChinaId) Birthday() (time.Time, string) {
	return id.birthday, id.raw[6:14]
}

func (id *ChinaId) Male() bool {
	return !id.female
}

func (id *ChinaId) Female() bool {
	return id.female
}
