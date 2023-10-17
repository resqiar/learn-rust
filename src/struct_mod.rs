#[derive(Debug)]
struct Color(u32, u32, u32);

#[derive(Debug)]
struct Vehicle {
    id: usize,
    name: String,
    color: Color,
}

struct Dimension(u32, u32);

struct Rect {
    name: String,
    color: Color,
    dimension: Dimension,
}

impl Rect {
    fn square(name: String, color: Color, size: u32) -> Self {
        return Self {
            name,
            color,
            dimension: Dimension(size, size),
        };
    }

    fn format_name(&self) -> String {
        return self.name.to_lowercase();
    }

    fn area(&self) -> u32 {
        return self.dimension.0 * self.dimension.1;
    }

    fn can_hold(&self, other: Rect) -> bool {
        return self.dimension.0 > other.dimension.0 && self.dimension.1 > other.dimension.1;
    }
}

pub fn main() {
    let mut car = Vehicle {
        id: 1,
        name: String::from("Car"),
        color: Color(255, 255, 255),
    };

    let boat = Vehicle {
        id: 1,
        name: String::from("Boat"),
        color: Color(255, 0, 0),
    };

    let car2 = Vehicle {
        id: 2,
        name: String::from("Lambo"),
        ..car
    };

    car.color = Color(0, 255, 0);

    println!("{:?}", car);
    println!("{:?}", car2);
    println!("{:?}", boat);

    let box1 = Rect {
        name: String::from("Front Box"),
        dimension: Dimension(10, 15),
        color: Color(255, 255, 255),
    };

    let box2 = Rect {
        name: String::from("Smaller Box"),
        dimension: Dimension(5, 5),
        color: Color(0, 0, 0),
    };

    // square implemented from Rectangle struct
    let square1 = Rect::square(String::from("Small Square"), Color(0, 0, 0), 5);

    let area = box1.area();
    let box_title = box1.format_name();

    println!("{}", area);
    println!("{}", box_title);
    println!("{}", box1.can_hold(box2));
    println!("{}", box1.can_hold(square1));
}
