use fatoora_derive::Validate;

#[derive(Validate)]
struct Payload<'a, T, const N: usize>
where
    T: Copy,
{
    #[validate(non_empty)]
    name: &'a String,
    values: [T; N],
}

fn main() {
    let name = String::from("seller");
    let payload = Payload::new(&name, [7_u8; 2]).unwrap();
    assert!(std::ptr::eq(payload.name, &name));
    assert_eq!(payload.values, [7, 7]);
    let blank = String::from(" \t");
    assert!(Payload::new(&blank, [7_u8; 2]).is_err());
}
