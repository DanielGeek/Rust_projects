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