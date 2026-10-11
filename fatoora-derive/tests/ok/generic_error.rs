use fatoora_derive::Validate;
use std::marker::PhantomData;

#[derive(Debug)]
struct Failure<T>(String, PhantomData<T>);
impl<T> From<String> for Failure<T> {
    fn from(message: String) -> Self {
        Self(message, PhantomData)
    }
}

#[derive(Validate)]
#[validate_error(Failure<T>)]
struct Payload<T> {
    #[validate(non_empty)]
    name: String,
    marker: PhantomData<T>,
}

fn main() {
    let error = Payload::<u8>::new(" ".into(), PhantomData).err().unwrap();
    assert_eq!(error.0, "name must be non-empty");
    let payload = Payload::<u8>::new("seller".into(), PhantomData).unwrap();
    assert_eq!(payload.name, "seller");
}
