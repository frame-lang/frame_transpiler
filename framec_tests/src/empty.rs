include!(concat!(env!("OUT_DIR"), "/", "empty.rs"));

#[cfg(test)]
mod tests {

    #[test]
    fn empty_state_machine_compiles() {
        assert!(true);
    }
}
