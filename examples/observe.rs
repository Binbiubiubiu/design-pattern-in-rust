trait IObserver {
    fn update(&self);
}

trait ISubject<'a, T: IObserver> {
    fn add(&mut self, observer: &'a T);
    fn del(&mut self, observer: &'a T);
    fn notify(&self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}

impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    fn new() -> Subject<'a, T> {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn add(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn del(&mut self, observer: &'a T) {
        let option_idx = self.observers.iter().position(|x| *x == observer);
        match option_idx {
            Some(idx) => {
                self.observers.remove(idx);
            }
            None => {}
        }
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update();
        }
    }
}

#[derive(PartialEq)]
struct Observer {
    name: String,
}

impl IObserver for Observer {
    fn update(&self) {
        println!("observer {} get the message", self.name);
    }
}

fn main() {
    let mut subject = Subject::new();
    let yz = Observer {
        name: "yz".to_string(),
    };
    let yzzy = Observer {
        name: "yzzy".to_string(),
    };
    subject.add(&yz);
    subject.add(&yzzy);
    subject.del(&yz);
    subject.notify();
}
