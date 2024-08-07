## fqs - file queries

[![test](https://github.com/EngineeringSoftware/fqs/actions/workflows/test.yml/badge.svg)](https://github.com/EngineeringSoftware/fqs/actions/workflows/test.yml)
[![coverage](https://github.com/EngineeringSoftware/fqs/actions/workflows/coverage.yml/badge.svg)](https://github.com/EngineeringSoftware/fqs/actions/workflows/coverage.yml)
[![fmt](https://github.com/EngineeringSoftware/fqs/actions/workflows/fmt.yml/badge.svg)](https://github.com/EngineeringSoftware/fqs/actions/workflows/fmt.yml)

`fqs` is a command line tool for writing queries over files. Query
syntax is inspired by SQL. In many ways `fqs` is similar to one (or
combination) of the following: `cut`, `paste`, `bc`, `grep`, and
`awk`, but `fqs` provides SQL-like declarative query language.

Here is an example command that prints the values from the second
column for each row if a value in the first column is greater than
1000.

```
fqs "select str(@1) from path/to/file where int(@0) > 1000"
```

`fqs` uses ' ' as a delimiter of columns in the given file.  (This is
equivalent to `cut -d' '`.)

## Examples

This section provides several examples. Let's say that we have the
following file (`demo.txt`).

```
33 77.0 true text
44 88.98 false more
77 123. true next
```

The next command prints all the values from the first column of the
file. Note that we use `@0` as a column reference.

```
fqs "select int(@0) from demo.txt"
```

The next command prints values from the first column doubled.

```
fqs "select int(@0) * 2 from demo.txt"
```

The next command prints values from the first column only if the third
column is `true`.

```
fqs "select int(@0) from demo.txt where bool(@2) = true"
```

The next command reorders columns, changes one of the columns from
lower-case letters to upper-case letters and processes only rows in
which the second column has value greater than 80.

```
fqs "select upper(str(@3)), sin(int(@0)) from demo.txt where float(@1) > 80"
```

The next command shows the number of rows, the sum of all values in
the first column, and the max value in the second column.

```
fqs "select count(1), sum(int(@0)), max(float(@1)) from demo.txt"
```

## Query language

At the moment, `fqs` supports the `select` statement.  In many ways
the `select` statement is similar to the one you might know from SQL,
but there are differences due to the nature of the data. Below is the
(approximate) grammar of the language:

```
Query ::= "select" Columns "from" Path ["where" Condition]
Columns ::= Aggs | Exprs
Aggs ::= AggFunc [,AggFunc]*
AggFunc ::= Id "(" CExpr ")" # see the list of functions later in this document
Exprs ::= CExpr [, CExpr]*
CExpr ::= ScaFunc | AExprs
AExprs ::= MExprs [Aop AExprs]
MExprs ::= MExprs [Mop Operand] | Operand
ScaFunc ::= Id "(" CExpr ")" # see the list of functions later in this document
Path ::= path to a file that contains data to process
Condition ::= WExp
WExp ::= Operand Lop Operand
Operand ::= Cast | Int | Float | Bool | String
Cast ::= Type "(" ColRef ")"
Type ::= "int" | "float" | "bool" | "str"
ColRef ::= "@"Int
Int ::= int constant
Float ::= float constant
Bool ::= "true" | "false"
String ::= "'"string"'"
Aop ::= "+" | "-"
Mop ::= "*" | "/"
Lop ::= "<" | ">" | "<=" | ">=" | "=" | "!="
Id ::= an identifier
```


### Keywords

This section contains the list of keywords.

`select`, `from`, `limit`, `where`, `int`, `float`, `str`, `bool`,
`true`, `false`.


### Scalar functions

This section contains the list of scalar functions.  All functions in
this section report an error if the given argument has an incorrect
type.

#### upper(str)

* Returns a string in which all lower-case characters are converted to
their upper-case equivalent. It returns null if the argument is null.

#### lower(str)

* Returns a string in which all upper-case characters are converted to
their lower-case equivalent. It returns null if the argument is null.

#### length(str)

* Returns the number of characters in the string argument. It returns
null if the argument is null.

#### rev(str)

* Reverses the given argument string. It returns null if the argument
is null.

#### abs(int|float)

* Computes the absolute value of the argument. It returns null if the
argument is null.

#### sign(int|float)

* Returns the sign of the given numerical argument. It returns null if
the argument is null.

#### ceil(int|float)

* Round the given number to an integer greater than or equal to the
input number. It returns null if the argument is null.

#### floor(int|float)

* Returns integer value less than or equal to the given argument. It
returns null if the argument is null.

#### round(int|float)

* Rounds the given argument numeric value to integer. It returns null
if the argument is null.

#### cos(int|float)

* Returns the cosine of the given numeric argument in radians. It
returns null if the argument is null.

#### sin(int|float)

* Returns the sine of the given numeric argument in radians. It
returns null if the argument is null.

### Aggregate functions

This section contains the list of aggregate functions.

#### sum(int|float)

* Returns the sum of non-null values.

#### count(any)

* Returns the number non-null values.

#### max(int|float)

* Finds the max numerical value. Null values are ignored.

#### min(int|float)

* Finds the min numerical value. Null values are ignored.

#### avg(int|float)

* Computes the average value. Null values are ignored.


## Contributing

Please check [this page](CONTRIBUTING.md).


## License

[BSD-3-Clause license](LICENSE).


## Contact

Feel free to get in touch if you have any comments: Milos Gligoric
`<milos.gligoric@gmail.com>`.
