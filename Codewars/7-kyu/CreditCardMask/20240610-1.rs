use std::cmp::max;

/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    let i = max(0, cc.len() as i32 - 4) as usize;
    let masks = (0..i).fold(String::new(), |acc, _| acc + "#");
    let unmasks = &cc[i..];
    masks + unmasks
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
