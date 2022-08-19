use std::collections::HashMap;

pub const MAX_ARG_COUNT: usize = 'Z' as usize - 'A' as usize + 1;

pub fn env(args: Vec<bool>) -> HashMap<char, bool> {
    assert!(args.len() <= MAX_ARG_COUNT);
    ('A'..).zip(args).collect()
}
