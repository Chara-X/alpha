package rdcloud

import (
	"database/sql"
	"encoding/csv"
	"io"
	"strings"

	_ "modernc.org/sqlite"
)

type DB struct{ *sql.DB }

func Open(dataSourceName string) *DB {
	var db, err = sql.Open("sqlite", dataSourceName)
	if err != nil {
		panic(err)
	}
	return &DB{db}
}
func (db *DB) Import(r io.Reader, table string) {
	var cr = csv.NewReader(r)
	var rows = []string{}
	var cols, _ = cr.Read()
	for row, err := cr.Read(); err == nil; row, err = cr.Read() {
		rows = append(rows, "('"+strings.Join(row, "','")+"')")
	}
	if _, err := db.Exec("INSERT INTO " + table + " (" + strings.Join(cols, ",") + ") VALUES " + strings.Join(rows, ",")); err != nil {
		panic(err)
	}
}
func (db *DB) Export(w io.Writer, query string) {
	var rows, _ = db.Query(query)
	var cols, _ = rows.Columns()
	var cw = csv.NewWriter(w)
	cw.Write(cols)
	for rows.Next() {
		var dest = make([]any, len(cols))
		for i := range cols {
			dest[i] = new(sql.NullString)
		}
		rows.Scan(dest...)
		var row = make([]string, len(cols))
		for i, v := range dest {
			if v, ok := v.(*sql.NullString); ok && v.Valid {
				row[i] = v.String
			}
		}
		cw.Write(row)
	}
	cw.Flush()
}
