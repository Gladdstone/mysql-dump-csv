# SQL Dump to CSV Converter

MySQL generates a dump file with mysqldump. This is a simple Rust program that parses the dump file and converts the inserted values into a CSV file.

## Generating the dump file

To generate a compatible SQL dump file from MySQL, use the `mysqldump` command.

### Basic Table Dump

```bash
mysqldump -u username -p database_name table_name > dump.sql
```

---

## Features

- Reads a `.sql` dump file
- Extracts column headers from the `INSERT INTO` statement
- Cleans SQL syntax (parentheses, commas, quotes, semicolons)
- Writes the parsed data to a `.csv` file

---

## How It Works

1. Opens a file passed as an argument or `dump.sql`
2. Looks for an `INSERT INTO` statement
3. Extracts column names from the header
4. Parses each row of inserted values
5. Cleans each value using a regular expression
6. Writes the results to `dump.csv`

---

## Input

- **File**
- Must contain at least one `INSERT INTO` statement
- Expected format (simplified):

```sql
INSERT INTO table_name (col1, col2, col3)
VALUES
('a', 'b', 'c'),
('d', 'e', 'f');

