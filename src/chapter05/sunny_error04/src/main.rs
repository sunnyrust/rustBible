use std::io;
fn main() {
    let ri = read_issue();
    match ri {
        Ok(issue) => {
            println!("{}", issue);
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}

fn read_issue() -> Result<String, io::Error> {
    Ok(std::fs::read_to_string("/etc/issue")?)
}
