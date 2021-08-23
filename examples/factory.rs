trait Human{
    fn eat(&self);
}

#[derive(Debug)]
enum HumanType{
    American,
    Chinese,
    Japanese,
}

struct American;

impl Human for American {
    fn eat(&self) {
        println!("American like bread");
    }
}

struct Chinese;

impl Human for Chinese {
    fn eat(&self){
        println!("Chinese like rice");
    }
}

struct Japanese;

impl Human for Japanese {
    fn eat(&self){
        println!("Japanese like rice");
    }
}

struct HumanFactory;

impl HumanFactory{
    fn new_human(s:&HumanType)->Box<dyn Human>{
        match s{
            HumanType::American => Box::new(American),
            HumanType::Chinese =>Box::new(Chinese),
            HumanType::Japanese => Box::new(Japanese),
        }
    }
}

fn main() {

    let human1 = HumanFactory::new_human(&HumanType::Chinese);
    let human2 = HumanFactory::new_human(&HumanType::American);
    let human3 = HumanFactory::new_human(&HumanType::Japanese);

    human1.eat();
    human2.eat();
    human3.eat();
}