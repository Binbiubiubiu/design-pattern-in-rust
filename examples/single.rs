use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use std::thread;

#[derive(Debug)]
struct Config {
    config_str: String,
}

// 单例函数
// 第一 唯一
// 第二 用Maybeunit创建 未被初始化的内存空间
fn single_config() -> &'static Mutex<Config> {
    static mut CONFIG: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        CONFIG.as_mut_ptr().write(Mutex::new(Config {
            config_str: "test config".to_string(),
        }));
    });
    unsafe { &*CONFIG.as_ptr() }
}

fn main() {
    let config1 = single_config();
    let config2 = single_config();
    println!("{:?}", config1);
    println!("{:?}", config2);

    {
        let mut conf = config1.lock().unwrap();
        conf.config_str = "config1".to_string();
    }
    println!("{:?}", config1);
    println!("{:?}", config2);

    let handle = thread::spawn(move || {
        let mut conf = single_config().lock().unwrap();
        conf.config_str = "thread change".to_string();
    });

    handle.join().unwrap();
    println!("{:?}", config1);
    println!("{:?}", config2);
}
