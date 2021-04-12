#[derive(Debug)]
pub enum Gender {
    Male, Female
}

#[derive(Debug)]
struct Employee {
    name: String,
    email: String,
    age: u8,
    gender:  Gender,
}
fn main() {
    let mut sunny_employee = Employee {
        name: String::from("Sunny"),
        email: String::from("sunny@sunny.com"),
        age: 46,
        gender: Gender::Male
    };
    println!("Employee : {:#?}", sunny_employee);
    let email=&sunny_employee.email;
    println!("Employee's Email : {:#?}", email);
    sunny_employee.gender=Gender::Female;

    let name = String::from("Rust");
    let age  = 10;
    let  sunny_employee2 = Employee {
        name ,
        email: String::from("sunny@sunny.com"),
        age,
        gender: Gender::Male
    };
    println!("Employee : {:#?}", sunny_employee2);

    let sunny_employee3=Employee{
        name: String::from("Alex"),
        email: String::from("Alex@sunny.com"),
        age: 36,
        ..sunny_employee2
    };

    println!("Employee : {:#?}", sunny_employee3);

    let sunny_employee4=Employee{
        ..sunny_employee3
    };

    println!("Employee : {:#?}", sunny_employee4);
}
