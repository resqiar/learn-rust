#[derive(Debug)]
enum VType {
    Electric,
    Diesel,
    Petrol,
}

enum Vehicle {
    Car(VType),
    Motorbike,
    ATV(Option<ATVExtensions>),
}

struct ATVExtensions {
    name: String,
}

pub fn main() {
    let some_number = Some(5);
    let some_char = Some('A');
    let absent_num: Option<u32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_num);

    let mercedes = Vehicle::Car(VType::Diesel);
    let car_id = get_registration_id(&mercedes);
    let mobike_id = get_registration_id(&Vehicle::Motorbike);
    let atv_id = get_registration_id(&Vehicle::ATV(None));

    println!("{}", car_id);
    println!("{}", mobike_id);
    println!("{}", atv_id);
    print_car_type(&mercedes);

    let atv_ext = Some(ATVExtensions {
        name: String::from("200ft Exhaust"),
    });
    let atv = Vehicle::ATV(atv_ext);

    if let Err(e) = get_atv_ext(&mercedes) {
        println!("Something went wrong: {}", e);
    }

    match get_atv_ext(&atv) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}

fn get_atv_ext(vehicle: &Vehicle) -> Result<String, std::io::Error> {
    match vehicle {
        Vehicle::ATV(Some(ext)) => Ok(String::from(&ext.name)),
        Vehicle::ATV(None) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "ATV Extension not found",
        )),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Input is not ATV",
        )),
    }
}

fn get_registration_id(vehicle: &Vehicle) -> String {
    match vehicle {
        Vehicle::Car(..) => String::from("car_a32-9_12"),
        Vehicle::Motorbike => String::from("mbike_2012x_79"),
        Vehicle::ATV(..) => String::from("atv_2209m_a8"),
    }
}

fn print_car_type(vehicle: &Vehicle) {
    match vehicle {
        Vehicle::Car(v_type) => {
            println!("{:?}", v_type);
        }
        _ => println!("Input is not Car."),
    }
}
