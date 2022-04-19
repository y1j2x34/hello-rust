pub mod util;

macro_rules! helper {
    () => {
        println!("I'm a helper!");
    }
}

#[macro_export]
macro_rules! helped {
    () => {
        $crate::helper!();
    }
}
