use std::rc::Rc;

trait Target {
    fn get_request(&self) -> String {
        String::from("Target: default behavior")
    }
}

struct CommonClient;
impl Target for CommonClient {}

struct Adapt{
    request_string:String
}

impl Adapt {
    pub fn new(request_string:String) -> Adapt{
        Adapt{request_string}
    }

    fn specific_req(&self)->String{
        format!("Adapt: {}",self.request_string)
    }
}

struct Adapter{
    adapt:Rc<Adapt>
}

impl Adapter {
    pub fn new(adapt:Rc<Adapt>) -> Adapter{
        Adapter{adapt}
    }
}

impl Target for Adapter{
    fn get_request(&self) -> String {
        format!("Adapter: {}",self.adapt.specific_req())
    }
}

fn run_demo<T:Target>(target:&T){
    println!("{}",target.get_request());
}

fn main() {
    println!("default target:");
    run_demo(&CommonClient);
    let adapt = Rc::new(Adapt::new("原子".to_string()));
    let adapter = Adapter::new(adapt);
    run_demo(&adapter);
}
