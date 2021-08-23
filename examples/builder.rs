#[derive(Clone)]
struct Warehouse {
    parts: Vec<String>,
}

impl Warehouse {
    fn new() -> Warehouse {
        Warehouse { parts: Vec::new() }
    }

    fn sell_parts(&self){
        for v in &self.parts{
            println!("{}",v);
        }
    }
}
trait Builder {
    fn fill_warehouset_part_a(&mut self);
    fn fill_warehouset_part_b(&mut self);
    fn name(&mut self) -> String;
    fn get_warehouse(&mut self) -> Warehouse;
}

struct HumanBuilder {
    warehouse: Warehouse,
}

impl HumanBuilder {
    fn new() -> HumanBuilder {
        HumanBuilder {
            warehouse: Warehouse::new(),
        }
    }
}

impl Builder for HumanBuilder {
    fn fill_warehouset_part_a(&mut self) {
        self.warehouse.parts.push("human product a part".to_string());
    }

    fn fill_warehouset_part_b(&mut self) {
        self.warehouse.parts.push("human product b part".to_string());
    }

    fn name(&mut self) -> String {
        "Human".to_string()
    }

    fn get_warehouse(&mut self) -> Warehouse {
        let w = self.warehouse.clone();
        self.warehouse = Warehouse::new();
        w
    }
}

struct MachineBuilder{
    warehouse:Warehouse,
}

impl MachineBuilder {
    fn new()->MachineBuilder {
        MachineBuilder{
            warehouse:Warehouse::new(),
        }
    }
}

impl Builder for MachineBuilder {
    fn fill_warehouset_part_a(&mut self) {
        self.warehouse.parts.push("machine product a part".to_string());
    }

    fn fill_warehouset_part_b(&mut self) {
        self.warehouse.parts.push("machine product b part".to_string());
    }


    fn name(&mut self) -> String {
        "Machine".to_string()
    }

    fn get_warehouse(&mut self) -> Warehouse {
        let w = self.warehouse.clone();
        self.warehouse = Warehouse::new();
        w
    }
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder }
    }

    fn fill_warehouse(&mut self) {
        self.builder.fill_warehouset_part_a();
        self.builder.fill_warehouset_part_b();
    }

    fn get_which_warehouse(&mut self) {
       println!("get {} type warehouse", self.builder.name());
    }

    fn get_warehouse(&mut self)-> Warehouse{
        self.builder.get_warehouse()
    }
}

fn main() {
    // 创建 实例builder特质 实例
    let human = HumanBuilder::new();
    let m = MachineBuilder::new();
    // 用builder 装入Director
    let mut driector = Director::new(Box::new(human));
    let mut director2 = Director::new(Box::new(m));
    // Director 进行指挥工作
    // 1 指挥Builder生产产品装入warehouse
    driector.fill_warehouse();
    // 2 获取装满通知
    driector.get_which_warehouse();
    // 3 获取warehouse
    let w = driector.get_warehouse();
    // 4 清空warehouse 产品
    w.sell_parts();

    director2.fill_warehouse();
    // 2 获取装满通知
    director2.get_which_warehouse();
    // 3 获取warehouse
    let w = director2.get_warehouse();
    // 4 清空warehouse 产品
    w.sell_parts();
}   
