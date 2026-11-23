package hanconv_test

import (
	"fmt"

	"github.com/fhluo/hanconv/go"
)

func ExampleS2T() {
	result := hanconv.S2T("简繁转换")
	fmt.Println(result)
	// Output: 簡繁轉換
}
