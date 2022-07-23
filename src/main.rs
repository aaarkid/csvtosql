use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn read_arguments () -> Result<(usize, String, String, String, Vec<String>), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let n = args.next().unwrap().parse::<usize>().unwrap();
    let input_file = args.next().unwrap();
    let output_file = args.next().unwrap();
    let table_name = args.next().unwrap();
    let args = args.collect::<Vec<String>>();
    if args.len() != n + 5 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid number of arguments")));
    }

    Ok((n, input_file, output_file, table_name, args))
}


fn read_csv_from_file (file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

fn create_sql_insert_statements(csv_data: &Vec<String>, table_name: &String, cols: &Vec<String>) -> Vec<String> {
    let mut sql_statements: Vec<String> = Vec::new();
    for line in csv_data {
        let fields: Vec<&str> = line.split(",").collect();
        let mut sql_fields: String = String::new();
        for field in cols {
            sql_fields.push_str(&format!("{}, ", field))
        }
        sql_fields.pop();
        sql_fields.pop();
        let mut sql_statement = String::from(&format!("INSERT INTO {} ({}) VALUES (", table_name, sql_fields));
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

fn write_file (queries: &Vec<String>, output: &String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output)?;
    for query in queries {
        file.write_all(query.as_bytes())?;
        file.write_all(";\n".as_bytes())?;
    }
    Ok(())
}

fn main() {
    let args = read_arguments().unwrap();
    let csv = match read_csv_from_file(&args.1) {
        Ok(csv) => csv,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    let sql_statements = create_sql_insert_statements(&csv, &args.3, &args.4);
    match write_file(&sql_statements, &args.2) {
        Ok(_) => println!("Success!"),
        Err(err) => println!("Error: {}", err),
    }
}
