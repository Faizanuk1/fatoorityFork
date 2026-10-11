use fatoora_derive::Validate;

#[derive(Validate)]
#[validate_error()]
struct Payload {
    name: String,
}

fn main() {}
