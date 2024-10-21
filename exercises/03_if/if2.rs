fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish.starts_with("fizz") {
        "foo"
    } else if fizzish.starts_with("fuzz") {
        "bar"
    } else {
        "baz"
    }
}

fn main() {
    assert_eq!(foo_if_fizz("fizz"), "foo");
    assert_eq!(foo_if_fizz("fuzz"), "bar");
    assert_eq!(foo_if_fizz("literally anything"), "baz");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        // This means that calling `foo_if_fizz` with the argument "fizz" should return "foo".
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}
