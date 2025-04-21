use js_sys::Uint32Array;
use std::fmt;

pub fn check_uint32array(
    array: &Uint32Array,
    expected: &[u32],
) -> Result<(), Box<dyn std::error::Error>> {
    let len = array.length();
    let mut actual = vec![0; len as usize];
    array.copy_to(&mut actual);

    assert_eq!(actual, expected);

    Ok(())
}

/// Logs a message to the browser console. Equivalent to "console.log".
pub fn log(args: &fmt::Arguments) {
    let s = fmt::format(*args);
    web_sys::console::log_1(&s.into());
}
