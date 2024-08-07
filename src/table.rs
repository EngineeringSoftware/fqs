use crate::errors::FqError;

pub struct Table {
    content: Vec<Vec<String>>,
}

pub struct TableIterator<'a> {
    table: &'a Table,
    row: usize,
}

impl Table {
    /// Creates an empty table.
    pub fn new() -> Table {
        Table {
            content: Vec::new(),
        }
    }

    pub fn iter(&self) -> TableIterator {
        TableIterator {
            table: self,
            row: 0,
        }
    }

    /// Creates a table from the given content. If number of columns
    /// is not the same in each row, we append values to each row to
    /// match the max columns.
    pub fn from(mut content: Vec<Vec<String>>) -> Table {
        if let Some(max_columns) = content.iter().map(|x| x.len()).max() {
            // We append "" to each row that does not have sufficient
            // columns, so each row ends up with the same number of
            // elements.
            for vec in &mut content {
                while vec.len() < max_columns {
                    // `null` is ""
                    vec.push(String::from(""));
                }
            }
            Table { content }
        } else {
            Table::new()
        }
    }

    /// Returns true if this is an empty table.
    pub fn empty(&self) -> bool {
        self.nrows() == 0 && self.ncols() == 0
    }

    pub fn non_empty(&self) -> bool {
        !self.empty()
    }

    /// Print table.
    pub fn show(&self) {
        for outer in &self.content {
            let joined = outer.join(" ");
            println!("{}", joined);
        }
    }

    /// Returns all values from the row at ix.
    pub fn row(&self, ix: usize) -> Result<Vec<String>, FqError> {
        if ix >= self.nrows() {
            return Err(FqError::exe("out of bounds: no such row"));
        }

        Ok(self.content[ix].clone())
    }

    /// Returns all values from the column at ix.
    pub fn col(&self, ix: usize) -> Result<Vec<String>, FqError> {
        if ix >= self.ncols() {
            return Err(FqError::exe("out of bounds: no such col"));
        }

        let mut col: Vec<String> = Vec::new();
        for row in &self.content {
            col.push(row[ix].to_string());
        }
        Ok(col)
    }

    /// Adds a column to the table (as the last column).
    pub fn push_col(&mut self, col: Vec<String>) -> Result<(), FqError> {
        if self.non_empty() && self.nrows() != col.len() {
            return Err(FqError::exe("Incorrect number of rows"));
        }

        if self.nrows() == 0 {
            for v in col {
                let mut row: Vec<String> = Vec::new();
                row.push(v);
                self.content.push(row);
            }
        } else {
            for (ix, row) in self.content.iter_mut().enumerate() {
                row.push(col[ix].to_string());
            }
        }

        Ok(())
    }

    pub fn push_row(&mut self, row: Vec<String>) -> Result<(), FqError> {
        if self.non_empty() && self.ncols() != row.len() {
            return Err(FqError::exe("Incorrect number of cols"));
        }

        self.content.push(row);
        Ok(())
    }

    /// Takes columns from the other table and pushes to this table.
    pub fn push_table(&mut self, other: &Table) -> Result<(), FqError> {
        for i in 0..=other.ncols() - 1 {
            self.push_col(other.col(i)?)?;
        }

        Ok(())
    }

    /// Returns the number of rows in the table.
    pub fn nrows(&self) -> usize {
        self.content.len()
    }

    /// Returns the number of columns in the table.
    pub fn ncols(&self) -> usize {
        if self.content.len() == 0 {
            0
        } else {
            self.content[0].len()
        }
    }
}

impl<'a> Iterator for TableIterator<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.table.row(self.row) {
            Ok(v) => {
                self.row += 1;
                Some(v)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_rows() {
        let table = Table::from(vec![]);
        assert_eq!(table.nrows(), 0);
    }

    #[test]
    fn number_of_cols() {
        let table = Table::from(vec![]);
        assert_eq!(table.ncols(), 0);
    }

    #[test]
    fn from_vec() {
        let vec = vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("bde"), String::from("boo")],
        ];

        let table = Table::from(vec);
        assert_eq!(table.nrows(), 2);
        assert_eq!(table.ncols(), 2);
    }

    #[test]
    fn from_vec_uneven() {
        let vec = vec![
            vec![String::from("abc")],
            vec![String::from("1"), String::from("2"), String::from("3")],
        ];

        let table = Table::from(vec);
        assert_eq!(2, table.nrows());
        assert_eq!(3, table.ncols());
    }

    #[test]
    fn row() {
        let vec = vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("bde"), String::from("boo")],
        ];

        let table = Table::from(vec);
        match table.row(0) {
            Ok(row) => {
                assert_eq!(row, vec!["abc", "def"]);
            }
            Err(err) => {
                panic!("could not get a row {err}");
            }
        }

        if let Ok(_) = table.row(10) {
            panic!("should given an error when accessing outside bounds");
        }
    }

    #[test]
    fn col() {
        let vec = vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("bde"), String::from("boo")],
        ];

        let table = Table::from(vec);
        match table.col(0) {
            Ok(col) => {
                assert_eq!(col, vec!["abc", "bde"]);
            }
            Err(err) => {
                panic!("could not get a column {err}");
            }
        }

        if let Ok(_) = table.col(10) {
            panic!("should given an error when accessing outside bounds");
        }
    }

    #[test]
    fn empty() {
        let table = Table::new();
        assert!(table.empty());

        let table = Table::from(vec![vec![String::from("abc"), String::from("def")]]);
        assert!(!table.empty());
    }

    #[test]
    fn push_col_into_empty_table() {
        let mut table = Table::new();

        let col = vec![String::from("abc"), String::from("def")];

        if let Err(FqError::Exe(msg)) = table.push_col(col) {
            panic!("Could not push a column {msg}");
        }

        assert_eq!(2, table.nrows());
        assert_eq!(1, table.ncols());
    }

    #[test]
    fn push_col_into_table() {
        let mut table = Table::from(vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("123"), String::from("345")],
        ]);

        assert_eq!(2, table.nrows());
        assert_eq!(2, table.ncols());

        if let Err(FqError::Exe(msg)) =
            table.push_col(vec![String::from("new"), String::from("column")])
        {
            panic!("Could not push a column {msg}");
        }

        assert_eq!(2, table.nrows());
        assert_eq!(3, table.ncols());

        match table.col(2) {
            Ok(col) => {
                assert_eq!(col, vec![String::from("new"), String::from("column")]);
            }
            Err(err) => {
                panic!("Could not get a column {err}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "Push should fail")]
    fn push_col_incorrect_rows() {
        let mut table = Table::from(vec![vec![String::from("abc"), String::from("def")]]);

        table.push_col(vec![]).expect("Push should fail");
    }

    #[test]
    fn push_row_into_empty_table() {
        let mut table = Table::new();

        let row = vec![String::from("abc"), String::from("def")];

        if let Err(FqError::Exe(msg)) = table.push_row(row) {
            panic!("Could not push a row into an empty table {msg}");
        }

        assert_eq!(1, table.nrows());
        assert_eq!(2, table.ncols());
    }

    #[test]
    #[should_panic(expected = "Push should fail")]
    fn push_row_incorrect_cols() {
        let mut table = Table::from(vec![vec![String::from("abc"), String::from("def")]]);

        let row = vec![String::from("123")];

        table.push_row(row).expect("Push should fail");
    }

    #[test]
    fn push_table_into_empty() {
        let mut table = Table::new();

        let other = Table::from(vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("123"), String::from("456")],
        ]);

        if let Err(FqError::Exe(msg)) = table.push_table(&other) {
            panic!("Could not push a table into an empty table {msg}");
        }

        assert_eq!(2, table.ncols());
        assert_eq!(2, table.nrows());
    }

    #[test]
    fn push_table_into_table() {
        let mut table = Table::from(vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("123"), String::from("456")],
        ]);

        let other = Table::from(vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("123"), String::from("456")],
        ]);

        if let Err(FqError::Exe(msg)) = table.push_table(&other) {
            panic!("Could not push into a table {msg}");
        }

        assert_eq!(4, table.ncols());
        assert_eq!(2, table.nrows());
    }

    #[test]
    #[should_panic(expected = "Push should fail")]
    fn push_table_incorrect_rows() {
        let mut table = Table::from(vec![
            vec![String::from("abc"), String::from("def")],
            vec![String::from("123"), String::from("456")],
        ]);

        let other = Table::from(vec![vec![String::from("abc"), String::from("def")]]);

        table.push_table(&other).expect("Push should fail");
    }
}
