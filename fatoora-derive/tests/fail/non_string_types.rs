use fatoora_derive::Validate;

#[derive(Validate)]
struct Slice<'a> {
    #[validate(non_empty)]
    value: &'a [u8],
}

#[derive(Validate)]
struct Array {
    #[validate(non_empty)]
    value: [u8; 2],
}

fn main() {}
