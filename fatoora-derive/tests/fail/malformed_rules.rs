use fatoora_derive::Validate;

#[derive(Validate)]
struct Payload {
    #[validate(non_empty = true)]
    name: String,
}

fn main() {}
