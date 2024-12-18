package main

import (
	"fmt"

	"github.com/xuri/excelize/v2"
)

func main() {}
func ReadExcel() {
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
func WriteEXCEL() {
	f := excelize.NewFile()
	defer func() {
		if err := f.Close(); err != nil {
			fmt.Println(err)
		}
	}()
	// Create a new sheet.
	index, err := f.NewSheet("Sheet2")
	if err != nil {
		fmt.Println(err)
		return
	}
	// Set value of a cell.
	f.SetCellValue("Sheet2", "A2", "Hello world.")
	f.SetCellValue("Sheet1", "B2", 100)
	// Set active sheet of the workbook.
	f.SetActiveSheet(index)
	// Save spreadsheet by the given path.
	if err := f.SaveAs("Book1.xlsx"); err != nil {
		fmt.Println(err)
	}
}
