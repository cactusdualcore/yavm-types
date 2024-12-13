use crate::Type;

#[derive(Debug)]
pub struct Dynamic {
    ty: Type,
    _value: Box<[u8]>,
}

impl Dynamic {
    pub fn value_type(&self) -> &Type {
        &self.ty
    }
}
