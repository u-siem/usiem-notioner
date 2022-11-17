pub mod api;
pub mod client;
mod alerter;

pub use alerter::NotionAlert;

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        assert_eq!(3,1+2);
    }
}