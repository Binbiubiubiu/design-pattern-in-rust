trait Handler<'a> {
    fn make_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
    fn handle(&self, request: &str);
}

struct First<'a> {
    name: String,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> First<'a> {
    fn new(name: String) -> First<'a> {
        First { name, next: None }
    }
}

impl<'a> Handler<'a> for First<'a> {
    fn make_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }

    fn handle(&self, request: &str) {
        println!("{} get the request: {}", self.name, request);
        if let Some(v) = &self.next {
            v.handle(request);
        }
    }
}

struct Second<'a> {
    name: String,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Second<'a> {
    fn new(name: String) -> Second<'a> {
        Second { name, next: None }
    }
}

impl<'a> Handler<'a> for Second<'a> {
    fn make_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }

    fn handle(&self, request: &str) {
        println!("{} get the request: {}", self.name, request);
        if let Some(v) = &self.next {
            v.handle(request);
        }
    }
}

struct Client {}
impl<'a> Client {
    fn handle<T: Handler<'a>>(h: &T) {
        h.handle("request from client")
    }
}

fn main() {
    let mut first = First::new("first".to_string());
    let second = Second::new("second".to_string());
    first.make_next(&second);
    Client::handle(&first);
}
