// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        //match和if的区别，match表达式整体的返回值会作为函数的返回值 而if就得写return
        // if (value == 0) {
        //     return Err(CreationError::Zero);
        // } else if (value < 0) {
        //     return Err(CreationError::Negative);
        // }
        // Ok(PositiveNonzeroInteger(value as u64))
        match value {
            0 => Err(CreationError::Zero),
            x if x < 0 => Err(CreationError::Negative),
            _ => Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
