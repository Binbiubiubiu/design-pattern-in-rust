trait Subject {
    fn req(&self);
}

struct CommonSubject;

impl Subject for CommonSubject {
    fn req(&self) {
        println!("CommonSubject: handling req.");
    }
}

struct Proxy<'a> {
    real_subject: &'a CommonSubject,
}

impl<'a> Proxy<'a> {
    fn new(real_subject: &'a CommonSubject) -> Self {
        Proxy { real_subject }
    }

    fn check_auth(&self) -> bool {
        println!("Proxy: auth");
        true
    }

    fn log(&self) {
        println!("Proxy: log");
    }
}

impl<'a> Subject for Proxy<'a> {
    fn req(&self) {
        if self.check_auth() {
            self.real_subject.req();
            self.log();
        }
    }
}

fn run_demo<T: Subject>(subject: &T) {
    subject.req();
}

fn main() {
    run_demo(&CommonSubject);
    println!("++++++++++++++++++++++++++++++++");
    let proxy = Proxy::new(&CommonSubject);
    run_demo(&proxy);
}
