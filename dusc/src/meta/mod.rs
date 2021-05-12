use self::module::Module;

pub mod module;
pub mod package;

#[derive(Debug)]
pub struct StaticCell<T: 'static>(&'static T);

impl<T> StaticCell<T> {
    pub fn new(inner: T) -> Self {
        StaticCell(Box::leak(Box::new(inner)))
    }

    /// Returns a static reference to the contained value. This is incredibly unsafe - the user must maintain that
    /// no references are in use when this struct is dropped.
    pub unsafe fn get_ref(&self) -> &'static T {
        self.0
    }
}

impl<T> Drop for StaticCell<T> {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.0 as *const T as *mut T);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DuskPath<'i> {
    Root,
    Name(&'i str),
    Member {
        left: Box<DuskPath<'i>>,
        right: Box<DuskPath<'i>>,
    },
    Scope {
        left: Box<DuskPath<'i>>,
        right: Box<DuskPath<'i>>,
    },
}

#[derive(Debug)]
pub enum Item<'i> {
    Module(Module<'i>),
    // ...
}
