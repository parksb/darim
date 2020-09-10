macro_rules! some_if_true {
    ($test:expr => $true_expr:expr) => {
        if $test {
            Some($true_expr)
        } else {
            None
        }
    };
}
