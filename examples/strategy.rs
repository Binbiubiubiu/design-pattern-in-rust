trait Fly {
    fn fly(&self) {
        println!("fly~~~~~");
    }
}

struct CanFly;
impl Fly for CanFly {}

struct CanNotFly;
impl Fly for CanNotFly {
    fn fly(&self) {
        println!("Can't fly");
    }
}

struct Pig {
    fly_action: Box<dyn Fly>,
}

impl Pig {
    fn new(fly: Box<dyn Fly>) -> Pig {
        Pig { fly_action: fly }
    }
    fn fly(&self) {
        self.fly_action.fly();
    }

    fn set_fly_action(&mut self, fly_action: Box<dyn Fly>) {
        self.fly_action = fly_action;
    }
}

fn main() {
    let mut pig_model = Pig::new(Box::new(CanNotFly));
    pig_model.fly();
    pig_model.set_fly_action(Box::new(CanFly));
    pig_model.fly();
}
