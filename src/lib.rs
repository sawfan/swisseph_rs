//use swisseph
//use libswisseph_sys as raw;

pub mod types;
pub mod util;
pub mod swe;
pub mod swe2;
pub mod body;
pub mod position;
pub mod zodiacal;

pub use types::*;
pub use util::*;
pub use body::*;
pub use position::*;
pub use zodiacal::*;

//pub mod old;
//pub use old::*;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }

}


