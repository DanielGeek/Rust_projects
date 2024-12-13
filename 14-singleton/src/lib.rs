#![allow(dead_code)]

mod step1 {
    pub static mut SINGLETON: Singleton = Singleton::new(42);

    pub struct Singleton {
        inner: u32,
    }

    impl Singleton {
        pub const fn new(value: u32) -> Self {
            Singleton { inner: value }
        }

        pub fn instance() -> *mut u32 {
            unsafe { &raw mut SINGLETON.inner }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn singleton() {
            unsafe {
                let instance1 = Singleton::instance();
                let instance2 = &raw mut SINGLETON;

                assert_eq!(*instance1, (*instance2).inner);
            }
        }
    }
}

mod step2 {
    pub static mut SINGLETON: Singleton = Singleton::new();

    pub struct Singleton {
        inner: Option<u32>,
    }

    impl Singleton {
        pub const fn new() -> Self {
            Singleton { inner: None }
        }

        fn init(&mut self, value: u32) {
            self.inner = Some(value);
        }

        pub fn instance() -> *mut Option<u32> {
            unsafe { &raw mut SINGLETON.inner }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn singleton() {
            unsafe {
                let instance1 = Singleton::instance();
                let instance2 = &raw mut SINGLETON;

                *instance1 = Some(42);

                assert_eq!(*instance1, (*instance2).inner);
            }
        }
    }
}

mod step3 {
    use std::sync::{Mutex, Once};

    pub static SINGLETON: Mutex<Singleton> = Mutex::new(Singleton::new(|| 0));
    pub static ONCE: Once = Once::new();

    pub struct Singleton {
        inner: Option<u32>,
        init: fn() -> u32,
    }

    impl Singleton {
        pub const fn new(init: fn() -> u32) -> Self {
            Singleton { inner: None, init }
        }

        pub fn instance() -> *mut u32 {
            ONCE.call_once(|| {
                let init = SINGLETON.lock().unwrap().init;
                let value = init();
                SINGLETON.lock().unwrap().inner = Some(value);
                println!("Initializing singleton value");
            });

            println!("Accessing singleton value");

            let mut singleton = SINGLETON.lock().unwrap();
            singleton.inner.as_mut().map_or_else(
                || std::ptr::null_mut(),
                |inner| inner as *mut u32,
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn singleton() {
            unsafe {
                let instance1 = Singleton::instance();
                let instance2 = Singleton::instance();
                assert_eq!(*instance1, *instance2);

                *instance1 = 42;
                assert_eq!(*instance1, *instance2);
            }
        }
    }
}