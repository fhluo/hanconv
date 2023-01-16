package conv

import "golang.org/x/exp/constraints"

// Min 返回序列中的最小值
func Min[T constraints.Ordered](data ...T) T {
	if len(data) == 0 {
		panic("序列为空，无法获得最小值")
	}

	min := data[0]
	for i := 1; i < len(data); i++ {
		if data[i] < min {
			min = data[i]
		}
	}

	return min
}
