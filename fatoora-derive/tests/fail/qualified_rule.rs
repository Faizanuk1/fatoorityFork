use fatoora_derive::Validate;

#[derive(Validate)]
struct Payload {
    #[validate(rules::non_empty)]
    name: String,
}

fn main() {}
