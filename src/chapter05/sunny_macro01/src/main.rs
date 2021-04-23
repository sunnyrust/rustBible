struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car =Car {
        color : String::from("Red"),
        transmission : Transmission::SemiAuto,
        convertible : true,
        mileage: 0u32,
    };
    
    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.mileage, 0);
    return car;
}
fn main() {
    let car01 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(car01.color, "Red");

    println!("{}", std::fs::read_to_string("/etc/issue").unwrap())
}
