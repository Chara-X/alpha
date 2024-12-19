package excel

import (
	"fmt"

	"github.com/xuri/excelize/v2"
)

func Read() {
	f, err := excelize.OpenFile("2024智算组件故障统计分析.xls")
	if err != nil {
		panic(err)
	}
	defer func() {
		// Close the spreadsheet.
		if err := f.Close(); err != nil {
			panic(err)
		}
	}()
	// Get all the rows in the Sheet1.
	rows, err := f.GetRows("导出结果")
	if err != nil {
		fmt.Println(err)
		return
	}
	for _, row := range rows {
		for _, colCell := range row {
			fmt.Print(colCell, "\t")
		}
		fmt.Println()
	}
}
