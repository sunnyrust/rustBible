use std::error::Error;

fn main() {
    let ri=read_issue();
    match ri {
        Ok(issue) => {
            println!("{}",issue);
        },
        Err(_err) => {
            println!("Failed to open the file.");
        }
    }
}

fn read_issue() -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string("/etc/issue")?)
}