use js_sys::Uint32Array;

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
