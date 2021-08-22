use std::fmt::Display;

#[derive(Debug)]
struct Statellite {
    name: String,
    velocity: f64,
}

#[derive(Debug)]
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for Statellite {
    fn describe(&self) -> String {
        format!(
            "{} flying at {}  miles per seconds",
            self.name, self.velocity
        )
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "{} have {} crew at {}",
            self.name, self.crew_size, self.altitude
        )
    }
}

impl Display for SpaceStation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{})",self.name,self.altitude)
    }
}

fn print_type<T: std::fmt::Debug>(item: T) {
    println!("{:?} is {}", item, std::any::type_name::<T>())
}

fn compare_and_print<T: Display + PartialEq + From<V>, V: Display + PartialEq + Copy>(a: T, b: V) {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is not equal to {}", a, b);
    }
}

fn get_displayable() -> impl Display {
    SpaceStation {
        name: "".to_string(),
        crew_size: 3,
        altitude: 234,
    }
}

fn main() {
    let h = Statellite {
        name: "ss".to_string(),
        velocity: 4.72,
    };
    h.describe();
    let s = SpaceStation {
        name: "s".to_string(),
        crew_size: 8,
        altitude: 89,
    };
    println!("{}", h.describe());
    println!("{}", s.describe());
    println!("{:?}", h);
    print_type(s);
    print_type(2i64);

    compare_and_print(1.0, 1);

    println!("{}",get_displayable());
}
