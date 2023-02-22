package hanzi

import "testing"

func TestMin(t *testing.T) {
	tests := []struct {
		data     []int
		expected int
	}{
		{[]int{9}, 9},
		{[]int{1, 3}, 1},
		{[]int{3, 1}, 1},
		{[]int{7, 6, 2, 3, 9}, 2},
	}

	for _, test := range tests {
		if r := Min(test.data...); r != test.expected {
			t.Errorf("Min(%v), got %v, want %v", test.data, r, test.expected)
		}
	}
}
