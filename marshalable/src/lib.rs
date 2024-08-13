pub use marshalable_derive::Marshalable;

pub trait Marshalable : Sized {
    fn try_unmarshal();
}

impl Marshalable for () {
    fn try_unmarshal() {
    }
}


#[derive(Debug, Default, Marshalable)]
struct HasUnitField {
    _b: (),
}
