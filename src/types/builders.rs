use super::{
    Array, Field, Function, IntType, Parameter, Struct, Type, TypeInfo, TypeKind, Variant,
};

/// Error type for builder operations.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Missing a required value.
    #[error("Missing required value: {0}")]
    MissingValue(&'static str),
    /// No type kind has been specified in the builder.
    #[error("No type kind specified.")]
    NoKind,
    /// Duplicate value encountered.
    #[error("Duplicate value encountered: {0}")]
    DuplicateValue(&'static str),
}

/// Builder for creating `Type` instances.
///
/// Allows constructing different kinds of types (e.g., structs, enums, arrays, etc.)
/// with a fluent interface.
pub struct TypeBuilder {
    name: String,
    kind: Option<TypeKind>,
}

impl TypeBuilder {
    /// Creates a new `TypeBuilder` with the given name.
    pub fn new<N: Into<String>>(name: N) -> Self {
        Self {
            name: name.into(),
            kind: None,
        }
    }

    /// Starts building an enumeration type.
    pub fn enumeration(&mut self) -> EnumBuilder {
        EnumBuilder::new(self)
    }

    /// Starts building a struct type.
    pub fn structure(&mut self) -> StructBuilder {
        StructBuilder::new(self)
    }

    /// Starts building an array type.
    pub fn array(&mut self) -> ArrayBuilder {
        ArrayBuilder::new(self)
    }

    /// Starts building a signed integer type.
    pub fn int(&mut self) -> IntBuilder {
        IntBuilder::new(self, true)
    }

    /// Starts building an unsigned integer type.
    pub fn uint(&mut self) -> IntBuilder {
        IntBuilder::new(self, false)
    }

    /// Starts building a function type.
    pub fn function(&mut self) -> FunctionBuilder {
        FunctionBuilder::new(self)
    }

    /// Tries to build the `Type`, returning a `Result`.
    ///
    /// Returns an error if no type kind has been set.
    pub fn try_build(self) -> Result<Type, BuilderError> {
        if let Some(kind) = self.kind {
            Ok(Type(Box::new(TypeInfo {
                name: Some(self.name),
                kind,
            })))
        } else {
            Err(BuilderError::NoKind)
        }
    }

    /// Builds the `Type`, panicking if no type kind has been set.
    ///
    /// # Panics
    /// Panics if no type kind has been set in the builder.
    pub fn build(self) -> Type {
        self.try_build().unwrap()
    }
}

/// Builder for creating enumeration types.
pub struct EnumBuilder<'a> {
    builder: &'a mut TypeBuilder,
    variants: Vec<Variant>,
}

impl<'a> EnumBuilder<'a> {
    /// Creates a new `EnumBuilder`.
    pub fn new(builder: &'a mut TypeBuilder) -> Self {
        Self {
            builder,
            variants: Vec::new(),
        }
    }

    /// Adds a variant to the enumeration.
    ///
    /// Returns an error if a duplicate variant is added.
    pub fn add_variant(mut self, variant: Variant) -> Result<Self, BuilderError> {
        if self.variants.iter().any(|v| v.name == variant.name) {
            Err(BuilderError::DuplicateValue("variant"))
        } else {
            self.variants.push(variant);
            Ok(self)
        }
    }

    /// Finishes building the enumeration type.
    pub fn finish(self) -> &'a mut TypeBuilder {
        self.builder.kind = Some(TypeKind::Enum(super::Enum {
            variants: self.variants,
        }));
        self.builder
    }
}

/// Builder for creating struct types.
pub struct StructBuilder<'a> {
    builder: &'a mut TypeBuilder,
    fields: Vec<Field>,
}

impl<'a> StructBuilder<'a> {
    /// Creates a new `StructBuilder`.
    pub fn new(builder: &'a mut TypeBuilder) -> Self {
        Self {
            builder,
            fields: Vec::new(),
        }
    }

    /// Adds a field to the struct.
    ///
    /// Returns an error if a duplicate field is added.
    pub fn add_field(mut self, field: Field) -> Result<Self, BuilderError> {
        if self.fields.iter().any(|f| f.name == field.name) {
            Err(BuilderError::DuplicateValue("field"))
        } else {
            self.fields.push(field);
            Ok(self)
        }
    }

    /// Finishes building the struct type.
    pub fn finish(self) -> &'a mut TypeBuilder {
        self.builder.kind = Some(TypeKind::Struct(Struct {
            fields: self.fields,
        }));
        self.builder
    }
}

/// Builder for creating array types.
pub struct ArrayBuilder<'a> {
    builder: &'a mut TypeBuilder,
    element_type: Option<Type>,
    len: Option<usize>,
}

impl<'a> ArrayBuilder<'a> {
    /// Creates a new `ArrayBuilder`.
    pub fn new(builder: &'a mut TypeBuilder) -> Self {
        Self {
            builder,
            element_type: None,
            len: None,
        }
    }

    /// Sets the element type of the array.
    pub fn set_element_type(mut self, element_type: Type) -> Self {
        self.element_type = Some(element_type);
        self
    }

    /// Sets the length of the array.
    pub fn len(mut self, len: usize) -> Self {
        self.len = Some(len);
        self
    }

    /// Finishes building the array type, returning a `Result`.
    pub fn try_finish(self) -> Result<&'a mut TypeBuilder, BuilderError> {
        let element_type = self
            .element_type
            .ok_or(BuilderError::MissingValue("element_type"))?;
        let len = self.len.ok_or(BuilderError::MissingValue("len"))?;

        self.builder.kind = Some(TypeKind::Array(Array { element_type, len }));
        Ok(self.builder)
    }

    /// Finishes building the array type, panicking on errors.
    ///
    /// # Panics
    /// Panics if an error occurs while finishing the builder.
    pub fn finish(self) -> &'a mut TypeBuilder {
        self.try_finish().unwrap()
    }
}

/// Builder for creating integer types.
pub struct IntBuilder<'a> {
    builder: &'a mut TypeBuilder,
    bits: Option<u16>,
    signed: bool,
}

impl<'a> IntBuilder<'a> {
    /// Creates a new `IntBuilder`.
    ///
    /// The `signed` parameter determines if the integer type is signed or unsigned.
    pub fn new(builder: &'a mut TypeBuilder, signed: bool) -> Self {
        Self {
            builder,
            bits: None,
            signed,
        }
    }

    /// Sets the number of bits for the integer type.
    ///
    /// # Panics
    /// Panics if the number of bits exceeds 4096.
    pub fn set_bits(mut self, bits: u16) -> Self {
        assert!(
            bits <= 4096,
            "The number of bits cannot exceed 4096, but got {}",
            bits
        );
        self.bits = Some(bits);
        self
    }

    /// Finishes building the integer type, returning a `Result`.
    pub fn try_finish(self) -> Result<&'a mut TypeBuilder, BuilderError> {
        let bits = self.bits.ok_or(BuilderError::MissingValue("bits"))?;
        let int_type = IntType { bits };

        self.builder.kind = Some(if self.signed {
            TypeKind::Int(int_type)
        } else {
            TypeKind::UInt(int_type)
        });
        Ok(self.builder)
    }

    /// Finishes building the integer type, panicking on errors.
    ///
    /// # Panics
    /// Panics if an error occurs while finishing the builder.
    pub fn finish(self) -> &'a mut TypeBuilder {
        self.try_finish().unwrap()
    }
}

/// Builder for creating function types.
pub struct FunctionBuilder<'a> {
    builder: &'a mut TypeBuilder,
    parameters: Vec<Parameter>,
    return_type: Option<Type>,
    body: Option<String>,
}

impl<'a> FunctionBuilder<'a> {
    /// Creates a new `FunctionBuilder`.
    pub fn new(builder: &'a mut TypeBuilder) -> Self {
        Self {
            builder,
            parameters: Vec::new(),
            return_type: None,
            body: None,
        }
    }

    /// Adds a parameter to the function.
    pub fn add_parameter(mut self, param: Parameter) -> Self {
        self.parameters.push(param);
        self
    }

    /// Sets the return type of the function.
    pub fn set_return_type(mut self, return_type: Type) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Sets the body of the function.
    pub fn set_body<S: Into<String>>(mut self, body: S) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Finishes building the function type, returning a `Result`.
    pub fn try_finish(self) -> Result<&'a mut TypeBuilder, BuilderError> {
        // Validation for required fields.
        let parameters = self.parameters;
        let return_type = self
            .return_type
            .ok_or(BuilderError::MissingValue("return_type"))?;

        self.builder.kind = Some(TypeKind::Function(Function {
            parameters,
            return_type,
            body: self.body,
        }));
        Ok(self.builder)
    }

    /// Finishes building the function type, panicking on errors.
    ///
    /// # Panics
    /// Panics if an error occurs while finishing the builder.
    pub fn finish(self) -> &'a mut TypeBuilder {
        self.try_finish().unwrap()
    }
}
