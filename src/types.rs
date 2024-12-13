mod visitors;
pub use visitors::Visitor;

mod builders;
pub use builders::TypeBuilder;

#[derive(Debug, Clone)]
pub struct Type(Box<TypeInfo>);

impl Type {
    #[must_use]
    pub fn builder(name: String) -> TypeBuilder {
        TypeBuilder::new(name)
    }

    pub fn name(&self) -> Option<&str> {
        self.0.name.as_deref()
    }

    pub fn visit<V: Visitor>(&self, visitor: &V) {
        match &self.0.kind {
            TypeKind::Enum(enumeration) => visitor.visit_enum(enumeration),
            TypeKind::Struct(structure) => visitor.visit_struct(structure),
            TypeKind::Int(int_type) => visitor.visit_int(int_type),
            TypeKind::UInt(int_type) => visitor.visit_uint(int_type),
            TypeKind::Array(array) => visitor.visit_array(array),
            TypeKind::Function(function) => visitor.visit_function(function),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    name: Option<String>,
    kind: TypeKind,
}

#[derive(Debug, Clone)]
enum TypeKind {
    Int(IntType),
    UInt(IntType),
    Enum(Enum),
    Struct(Struct),
    Array(Array),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct IntType {
    bits: u16,
}

impl IntType {
    pub fn bits(&self) -> u16 {
        self.bits
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    fields: Vec<Field>,
}

impl Struct {
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    name: Option<String>,
    ty: Type,
}

impl Field {
    pub fn new(name: Option<impl Into<String>>, ty: Type) -> Self {
        Self {
            name: name.map(Into::into),
            ty,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn field_type(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    variants: Vec<Variant>,
}

impl Enum {
    pub fn variants(&self) -> &[Variant] {
        &self.variants
    }
}

#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
}

impl Variant {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    element_type: Type,
    len: usize,
}

#[allow(clippy::len_without_is_empty)]
impl Array {
    pub fn element_type(&self) -> &Type {
        &self.element_type
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    parameters: Vec<Parameter>,
    return_type: Type,
    body: Option<String>,
}

impl Function {
    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
    ty: Type,
}

impl Parameter {
    pub fn new(name: impl Into<String>, ty: Type) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameter_type(&self) -> &Type {
        &self.ty
    }
}
