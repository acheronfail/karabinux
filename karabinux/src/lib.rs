pub mod event;
pub mod karabiner;
pub mod key_state;
pub mod pipe;
pub mod state;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
