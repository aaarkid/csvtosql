use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn read_csv_from_file (file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

fn create_sql_insert_statements(csv_data: &Vec<String>) -> Vec<String> {
    let mut sql_statements: Vec<String> = Vec::new();
    for line in csv_data {
        let fields: Vec<&str> = line.split(",").collect();
        let mut sql_statement = String::from("INSERT INTO RECORDS (NID, Name, Birthday, Phone, NIPT, DRT, Wage, Job, Subject) VALUES (");
        for field in fields {
            sql_statement.push_str(&format!("'{}', ", field));
        }
        sql_statement.pop();
        sql_statement.pop();
        sql_statement.push(')');
        sql_statements.push(sql_statement);
    }
    sql_statements
}

fn write_file (queries: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("ext/queries.sql")?;
    for query in queries {
        file.write_all(query.as_bytes())?;
        file.write_all(";\n".as_bytes())?;
    }
    Ok(())
}

fn main() {
    let csv = match read_csv_from_file("ext/Numrat.csv") {
        Ok(csv) => csv,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    let sql_statements = create_sql_insert_statements(&csv);
    match write_file(&sql_statements) {
        Ok(_) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }
}
