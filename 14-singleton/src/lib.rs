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

mod step4 {
    use std::mem::MaybeUninit;
    use std::sync::{Mutex, Once};

    pub static SINGLETON: Mutex<MaybeUninit<Singleton>> = Mutex::new(MaybeUninit::uninit());
    pub static ONCE: Once = Once::new();
    pub static INIT: Mutex<Option<fn() -> u32>> = Mutex::new(None);

    pub struct Singleton {
        inner: u32,
    }

    pub fn set_initializer(initializer: fn() -> u32) {
        let mut init_lock = INIT.lock().unwrap();
        *init_lock = Some(initializer);
    }

    pub fn instance() -> &'static mut u32 {
        ONCE.call_once(|| {
            let init = {
                let init_lock = INIT.lock().unwrap();
                if let Some(init_fn) = *init_lock {
                    init_fn
                } else {
                    panic!("Singleton must be initialized before it is used.");
                }
            };

            let value = init();
            let singleton = Singleton { inner: value };

            let mut singleton_lock = SINGLETON.lock().unwrap();
            singleton_lock.write(singleton);
        });

        let mut singleton_lock = SINGLETON.lock().unwrap();
        let singleton_ptr = singleton_lock.as_mut_ptr();
        unsafe { &mut (*singleton_ptr).inner }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn singleton() {
            set_initializer(|| 42);

            let instance1 = instance();
            let instance2 = instance();

            assert_eq!(*instance1, *instance2);

            *instance1 = 24;

            assert_eq!(*instance1, *instance2);
        }
    }
}

mod step5 {
    use std::mem::MaybeUninit;
    use std::sync::{Mutex, Once};

    pub static SINGLETON: Mutex<MaybeUninit<Singleton>> = Mutex::new(MaybeUninit::uninit());
    pub static ONCE: Once = Once::new();
    pub static INIT: Mutex<Option<fn() -> u32>> = Mutex::new(None);

    pub struct Singleton {
        inner: Mutex<u32>,
    }

    impl Singleton {
        pub fn set_initializer(initializer: fn() -> u32) {
            let mut init_lock = INIT.lock().unwrap();
            *init_lock = Some(initializer);
        }

        pub fn instance() -> &'static Singleton {
            ONCE.call_once(|| {
                let init = {
                    let init_lock = INIT.lock().unwrap();
                    if let Some(init_fn) = *init_lock {
                        init_fn
                    } else {
                        panic!("Singleton must be initialized before it is used.");
                    }
                };

                let value = init();

                let singleton = Singleton {
                    inner: Mutex::new(value),
                };

                let mut singleton_lock = SINGLETON.lock().unwrap();
                singleton_lock.write(singleton);
            });

            unsafe {
                &*SINGLETON.lock().unwrap().as_ptr()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn singleton() {
            Singleton::set_initializer(|| 42);

            let instance1 = Singleton::instance();
            let instance2 = Singleton::instance();

            assert_eq!(*instance1.inner.lock().unwrap(), 42);
            assert_eq!(*instance2.inner.lock().unwrap(), 42);

            *instance1.inner.lock().unwrap() = 24;

            assert_eq!(*instance2.inner.lock().unwrap(), 24);

            *instance2.inner.lock().unwrap() = 142;

            assert_eq!(*instance1.inner.lock().unwrap(), 142);
        }
    }
}
