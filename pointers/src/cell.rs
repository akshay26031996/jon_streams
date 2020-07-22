use std::cell::UnsafeCell;

// Cell does not implement sync
// If I had two threads that both
// have a immutable reference to the
// cell, then both threads could change
// the value of cell at the same time.
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// This syntax is nightly only.
// This can be done by using something
// in the struct that is already implementing
// !Sync. UnsafeCell already does that, hence
// we don't need to implement this. This is needed
// to fail the bad test.
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // I have checked that no one else is mutating
        // the value.
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    // use super::Cell;
    // fn bad() {
    // let x = std::sync::Arc::new(Cell::new(42));
    // nothing is preventing this
    // for this we have to specify
    // that x can't be shared among
    // threads
    // let x1 = std::sync::Arc::clone(&x);
    // `std::cell::UnsafeCell<i32>` cannot be shared between threads safely
    // std::thread::spawn(|| {
    //     x1.set(43);
    // });
    // let x2 = std::sync::Arc::clone(&x);
    // std::thread::spawn(|| {
    //     x2.set(44);
    // });
    // }
}
