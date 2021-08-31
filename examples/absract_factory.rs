trait SysFactory {
    fn create_kernel(&self) -> Box<dyn Kernel>;
    fn create_gui(&self) -> Box<dyn Gui>;
}

struct WinFactory;
impl SysFactory for WinFactory {
    fn create_kernel(&self) -> Box<dyn Kernel> {
        Box::new(WinKernel {})
    }

    fn create_gui(&self) -> Box<dyn Gui> {
        Box::new(WinGui {})
    }
}

struct MacFactory;
impl SysFactory for MacFactory {
    fn create_kernel(&self) -> Box<dyn Kernel> {
        Box::new(MacKernel {})
    }

    fn create_gui(&self) -> Box<dyn Gui> {
        Box::new(MacGui {})
    }
}

trait Kernel {
    fn create(&self);
}

trait Gui {
    fn paint(&self);
}

struct WinKernel;
impl Kernel for WinKernel {
    fn create(&self) {
        println!("NT create");
    }
}

struct WinGui;
impl Gui for WinGui {
    fn paint(&self) {
        println!("Gui win");
    }
}

struct MacKernel;
impl Kernel for MacKernel {
    fn create(&self) {
        println!("M1 create");
    }
}

#[derive(Debug)]
enum Sys {
    Mac,
    Win,
}

struct System;
impl System {
    fn new_sys(os: &Sys) -> Box<dyn SysFactory> {
        match os {
            Sys::Mac => Box::new(MacFactory {}),
            Sys::Win => Box::new(WinFactory {}),
        }
    }
}

struct MacGui;
impl Gui for MacGui {
    fn paint(&self) {
        println!("Gui mac");
    }
}

fn main() {
    let mac_system = System::new_sys(&Sys::Mac);
    let mac_kernel = mac_system.create_kernel();
    mac_kernel.create();
    let mac_gui = mac_system.create_gui();
    mac_gui.paint();
    println!("===============================================");
    let win_system = System::new_sys(&Sys::Win);
    let win_kernel = win_system.create_kernel();
    win_kernel.create();
    let win_gui = win_system.create_gui();
    win_gui.paint();
    println!("===============================================");
}
