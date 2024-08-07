use fqs;
use fqs::args::Args;

#[test]
fn select_all() {
    let query = "select * from tests/fixtures/types.txt";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(4, table.ncols());
}

#[test]
fn select_all_limit() {
    let query = "select * from tests/fixtures/types.txt limit 1";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(4, table.ncols());

    let row = table.row(0).expect("Could not get the row");
    assert_eq!(row, vec!["55", "true", "today", "55.0"]);
}

#[test]
fn select_all_where_int() {
    let query = "select * from tests/fixtures/types.txt where int(@0) = 55";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(4, table.ncols());

    let row = table.row(0).expect("Could not get the row");
    assert_eq!(row, vec!["55", "true", "today", "55.0"]);

    let query = "select * from tests/fixtures/types.txt where int(@0) != 55";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(2, table.nrows());
    assert_eq!(4, table.ncols());

    let row = table.row(0).expect("Could not get the row");
    assert_eq!(row, vec!["66", "false", "yesterday", "66.0"]);
}

#[test]
fn select_all_where_bool() {
    let query = "select * from tests/fixtures/types.txt where bool(@1) = true";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(2, table.nrows());
    assert_eq!(4, table.ncols());

    let row = table.row(0).expect("Could not get the row");
    assert_eq!(row, vec!["55", "true", "today", "55.0"], "Incorrect row");
}

#[test]
fn select_plus() {
    let query = "select int(@0) + 5 from tests/fixtures/types.txt";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["60", "71", "-10"]);
}

#[test]
fn select_mul() {
    let query = "select int(@0) * 3 from tests/fixtures/types.txt";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["165", "198", "-45"]);
}

#[test]
fn select_div() {
    let query = "select int(@0) / 5 from tests/fixtures/types.txt";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["11", "13", "-3"]);
}

#[test]
fn select_minus() {
    let query = "select int(@0) - 10 from tests/fixtures/types.txt";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["45", "56", "-25"]);
}

#[test]
fn select_with_negative_numbers() {
    let args = Args::new("select -1 * int(@0) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["-55", "-66", "15"]);
}

#[test]
fn select_with_string_literal() {
    let query = "select int(@0) from tests/fixtures/types.txt where str(@0) = '55'";
    let args = Args::new(query.to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["55"]);
}

#[test]
#[should_panic(expected = "Syntax error: Unknown char")]
fn select_err_incorrect_char() {
    let args = Args::new(";".to_string());
    if let Err(err) = fqs::query(args) {
        panic!("{err}");
    }
}

#[test]
#[should_panic(expected = "Parse error: It has extra tokens")]
fn select_err_file_path() {
    let args = Args::new("select 1 from where int(@0) > 10".to_string());
    if let Err(err) = fqs::query(args) {
        panic!("{err}");
    }
}

#[test]
#[should_panic(expected = "Parse error: Column reference has to be cast")]
fn select_err_column_ref_cast() {
    let args = Args::new("select @1".to_string());
    if let Err(err) = fqs::query(args) {
        panic!("{err}");
    }
}

#[test]
#[should_panic(expected = "Semantics error: File does not exist")]
fn select_err_missing_file() {
    let args = Args::new("select 1 from somethingthatdoesnotexist".to_string());
    if let Err(err) = fqs::query(args) {
        panic!("{err}");
    }
}

#[test]
fn select_abs_func() {
    let args = Args::new("select abs(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["55", "66", "15"]);
}

#[test]
fn select_upper_func() {
    let args = Args::new("select upper(str(@2)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["TODAY", "YESTERDAY", "TOMORROW"]);
}

#[test]
fn select_lower_func() {
    let args = Args::new("select lower(str(@2)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["today", "yesterday", "tomorrow"]);
}

#[test]
fn select_length_func() {
    let args = Args::new("select length(str(@2)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["5", "9", "8"]);
}

#[test]
fn select_sum_func() {
    let args = Args::new("select sum(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["106"]);
}

#[test]
fn select_count_func() {
    let args = Args::new("select count(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["3"]);
}

#[test]
fn select_max_func() {
    let args = Args::new("select max(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["66"]);
}

#[test]
fn select_min_func() {
    let args = Args::new("select min(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["-15"]);
}

#[test]
fn select_avg_func() {
    let args = Args::new("select avg(int(@0)) from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).unwrap();

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["35"]);
}

#[test]
fn select_from_empty() {
    let args = Args::new("select * from tests/fixtures/empty.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(0, table.nrows());
    assert_eq!(0, table.ncols());

    let args = Args::new("select str(@0) from tests/fixtures/empty.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(0, table.nrows());
    assert_eq!(0, table.ncols());
}

#[test]
fn select_from_nulls() {
    let args = Args::new("select * from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(4, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["true", "false", "true"]);

    let col = table.col(1).expect("Could not get the column");
    assert_eq!(col, vec!["33", "", "20"]);

    let col = table.col(2).expect("Could not get the column");
    assert_eq!(col, vec!["17.23", "50", ""]);

    let col = table.col(3).expect("Could not get the column");
    assert_eq!(col, vec!["", "15.77", ""]);
}

#[test]
fn select_mul_with_nulls() {
    let args = Args::new("select int(@1) * 2 from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["66", " ", "40"]);
}

#[test]
fn select_sign_with_nulls() {
    let args = Args::new("select sign(int(@1)) from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).expect("Could not get the column");
    assert_eq!(col, vec!["1", " ", "1"]);
}

#[test]
fn select_sum_func_with_nulls() {
    let args = Args::new("select sum(int(@1)) from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["53"]);
}

#[test]
fn select_max_func_with_nulls() {
    let args = Args::new("select max(int(@1)) from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["33"]);
}

#[test]
fn select_min_func_with_nulls() {
    let args = Args::new("select min(int(@1)) from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["20"]);
}

#[test]
fn select_count_func_with_nulls() {
    let args = Args::new("select count(int(@1)) from tests/fixtures/nulls.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["2"]);
}

#[test]
fn select_sum_from_empty() {
    let args = Args::new("select sum(int(@1)) from tests/fixtures/empty.txt".to_string());
    let table = fqs::query(args).expect("Failing query");

    assert!(table.empty());
}

// #[test]
// fn select_func_and_expr() {
//     let args = Args::new("select sum(int(@0)) * 2 from tests/fixtures/types.txt".to_string());
//     let table = fqs::query(args).unwrap();

//     assert_eq!(1, table.nrows());
//     assert_eq!(1, table.ncols());

//     let col = table.col(0).unwrap();
//     assert_eq!(col, vec!["212"]);
// }

#[test]
fn select_from_weird_file_name() {
    let args =
        Args::new(r"select sum(int(@0)) from tests/fixtures/s\ p\ a\ ces\(\).txt".to_string());
    let table = fqs::query(args).unwrap();

    assert_eq!(1, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["-2"]);
}

#[test]
fn select_with_where_expr() {
    let args = Args::new(
        "select int(@0) from tests/fixtures/types.txt where sin(int(@0)) > cos(int(@0))"
            .to_string(),
    );
    let table = fqs::query(args).unwrap();

    assert_eq!(2, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["66", "-15"]);
}

#[test]
fn select_with_multiple_exprs() {
    let args =
        Args::new("select abs(int(@0) / 2 / 5) /2 from tests/fixtures/types.txt".to_string());
    let table = fqs::query(args).unwrap();

    assert_eq!(3, table.nrows());
    assert_eq!(1, table.ncols());

    let col = table.col(0).unwrap();
    assert_eq!(col, vec!["2", "3", "0"]);
}
