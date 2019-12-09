#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod client;
pub mod response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::client::AirlyClient;
        assert_eq!(1, 1);
    }
}
