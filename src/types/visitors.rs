use super::{Array, Enum, Function, IntType, Struct};

pub trait Visitor {
    fn visit_enum(&self, enumeration: &Enum);

    fn visit_struct(&self, structure: &Struct);

    fn visit_int(&self, int_type: &IntType);

    fn visit_uint(&self, int_type: &IntType);

    fn visit_array(&self, array: &Array);

    fn visit_function(&self, function: &Function);
}
