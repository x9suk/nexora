use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents a type in the Nexora type system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    String,
    Bool,
    Null,
    Void,
    Never,

    // Compound types
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),

    // Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
        is_async: bool,
    },

    // Generic type
    Generic {
        name: String,
        bounds: Vec<TraitBound>,
    },

    // Type variable (for inference)
    Var(TypeVar),

    // User-defined types
    Class(ClassType),
    Enum(EnumType),
    Interface(InterfaceType),
    Trait(TraitType),

    // Reference types
    Reference(Box<Type>),
    MutableReference(Box<Type>),

    // Special types
    Unknown,
    Error,
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Int | Type::Float | Type::String | Type::Bool | Type::Null
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    pub fn is_comparable(&self) -> bool {
        matches!(
            self,
            Type::Int | Type::Float | Type::String | Type::Bool
        )
    }

    pub fn is_assignable_to(&self, target: &Type) -> bool {
        if self == target {
            return true;
        }

        match (self, target) {
            (Type::Int, Type::Float) => true,
            (Type::Null, Type::Reference(_)) => true,
            (Type::Null, Type::Array(_)) => true,
            (Type::Null, Type::Map(_, _)) => true,
            (Type::Array(a), Type::Array(b)) => a.is_assignable_to(b),
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                k1.is_assignable_to(k2) && v1.is_assignable_to(v2)
            }
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::Null => write!(f, "Null"),
            Type::Void => write!(f, "Void"),
            Type::Never => write!(f, "Never"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::Map(key, val) => write!(f, "{{{}: {}}}", key, val),
            Type::Tuple(types) => {
                let items: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "({})", items.join(", "))
            }
            Type::Function {
                params,
                return_type,
                is_async,
            } => {
                let param_str: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                let async_prefix = if *is_async { "async " } else { "" };
                write!(f, "{}fn({}) -> {}", async_prefix, param_str.join(", "), return_type)
            }
            Type::Generic { name, bounds } => {
                if bounds.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let bound_str: Vec<String> = bounds.iter().map(|b| b.to_string()).collect();
                    write!(f, "{}: {}", name, bound_str.join(" + "))
                }
            }
            Type::Var(var) => write!(f, "?{}", var.id),
            Type::Class(class) => write!(f, "{}", class.name),
            Type::Enum(enum_type) => write!(f, "{}", enum_type.name),
            Type::Interface(iface) => write!(f, "{}", iface.name),
            Type::Trait(trait_type) => write!(f, "{}", trait_type.name),
            Type::Reference(inner) => write!(f, "&{}", inner),
            Type::MutableReference(inner) => write!(f, "&mut {}", inner),
            Type::Unknown => write!(f, "Unknown"),
            Type::Error => write!(f, "Error"),
        }
    }
}

/// Type variable for type inference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeVar {
    pub id: usize,
    pub name: Option<String>,
}

/// Trait bound for generic types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraitBound {
    pub name: String,
    pub args: Vec<Type>,
}

impl fmt::Display for TraitBound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.name)
        } else {
            let args: Vec<String> = self.args.iter().map(|a| a.to_string()).collect();
            write!(f, "{}<{}>", self.name, args.join(", "))
        }
    }
}

/// Class type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClassType {
    pub name: String,
    pub type_params: Vec<String>,
    pub superclass: Option<Box<Type>>,
    pub implements: Vec<Type>,
    pub fields: HashMap<String, Type>,
    pub methods: HashMap<String, FunctionType>,
}

/// Function type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionType {
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub is_async: bool,
    pub is_method: bool,
    pub is_static: bool,
}

/// Enum type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumType {
    pub name: String,
    pub type_params: Vec<String>,
    pub variants: Vec<EnumVariant>,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<Vec<Type>>,
}

/// Interface type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceType {
    pub name: String,
    pub type_params: Vec<String>,
    pub methods: HashMap<String, FunctionType>,
    pub extends: Vec<Type>,
}

/// Trait type information
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraitType {
    pub name: String,
    pub type_params: Vec<String>,
    pub methods: HashMap<String, FunctionType>,
    pub associated_types: Vec<String>,
}

/// Type constraint for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeConstraint {
    /// Two types must be equal
    Equal(Type, Type),
    /// Type must be a subtype of another
    Subtype(Type, Type),
    /// Type must implement a trait
    Implements(Type, TraitBound),
    /// Type must be numeric
    Numeric(Type),
    /// Type must be comparable
    Comparable(Type),
    /// Type must be callable
    Callable {
        callee: Type,
        args: Vec<Type>,
        result: Type,
    },
    /// Type must be indexable
    Indexable {
        object: Type,
        index: Type,
        result: Type,
    },
}

/// Type inference context
pub struct TypeContext {
    variables: HashMap<String, Type>,
    types: HashMap<String, Type>,
    constraints: Vec<TypeConstraint>,
    type_var_counter: usize,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            variables: HashMap::new(),
            types: HashMap::new(),
            constraints: Vec::new(),
            type_var_counter: 0,
        }
    }

    /// Create a new type variable
    pub fn new_type_var(&mut self, name: Option<String>) -> Type {
        let id = self.type_var_counter;
        self.type_var_counter += 1;
        Type::Var(TypeVar { id, name })
    }

    /// Add a variable binding
    pub fn add_variable(&mut self, name: String, ty: Type) {
        self.variables.insert(name, ty);
    }

    /// Get variable type
    pub fn get_variable(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    /// Add a type definition
    pub fn add_type(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    /// Get type definition
    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    /// Add a type constraint
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.constraints.push(constraint);
    }

    /// Solve type constraints
    pub fn solve(&mut self) -> Result<HashMap<usize, Type>, TypeError> {
        let mut substitutions: HashMap<usize, Type> = HashMap::new();

        // Simple constraint solving - unify equal types
        for constraint in &self.constraints {
            match constraint {
                TypeConstraint::Equal(t1, t2) => {
                    self.unify(t1, t2, &mut substitutions)?;
                }
                TypeConstraint::Subtype(t1, t2) => {
                    if !t1.is_assignable_to(t2) {
                        return Err(TypeError::Mismatch(t1.clone(), t2.clone()));
                    }
                }
                TypeConstraint::Numeric(t) => {
                    if let Type::Var(var) = t {
                        substitutions.insert(var.id, Type::Int);
                    } else if !t.is_numeric() {
                        return Err(TypeError::NotNumeric(t.clone()));
                    }
                }
                TypeConstraint::Comparable(t) => {
                    if let Type::Var(var) = t {
                        substitutions.insert(var.id, Type::Int);
                    } else if !t.is_comparable() {
                        return Err(TypeError::NotComparable(t.clone()));
                    }
                }
                _ => {}
            }
        }

        Ok(substitutions)
    }

    /// Unify two types
    fn unify(
        &self,
        t1: &Type,
        t2: &Type,
        substitutions: &mut HashMap<usize, Type>,
    ) -> Result<(), TypeError> {
        // Apply existing substitutions
        let t1 = self.apply_substitutions(t1, substitutions);
        let t2 = self.apply_substitutions(t2, substitutions);

        match (&t1, &t2) {
            // Same type
            _ if t1 == t2 => Ok(()),

            // Type variable cases
            (Type::Var(var), _) => {
                if let Some(existing) = substitutions.get(&var.id) {
                    self.unify(existing, &t2, substitutions)
                } else {
                    substitutions.insert(var.id, t2);
                    Ok(())
                }
            }
            (_, Type::Var(var)) => {
                if let Some(existing) = substitutions.get(&var.id) {
                    self.unify(&t1, existing, substitutions)
                } else {
                    substitutions.insert(var.id, t1);
                    Ok(())
                }
            }

            // Array types
            (Type::Array(inner1), Type::Array(inner2)) => {
                self.unify(inner1, inner2, substitutions)
            }

            // Map types
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                self.unify(k1, k2, substitutions)?;
                self.unify(v1, v2, substitutions)
            }

            // Function types
            (
                Type::Function {
                    params: p1,
                    return_type: r1,
                    ..
                },
                Type::Function {
                    params: p2,
                    return_type: r2,
                    ..
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(TypeError::ArityMismatch(p1.len(), p2.len()));
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b, substitutions)?;
                }
                self.unify(r1, r2, substitutions)
            }

            // Tuple types
            (Type::Tuple(t1), Type::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(TypeError::ArityMismatch(t1.len(), t2.len()));
                }
                for (a, b) in t1.iter().zip(t2.iter()) {
                    self.unify(a, b, substitutions)?;
                }
                Ok(())
            }

            // Mismatched types
            _ => Err(TypeError::Mismatch(t1, t2)),
        }
    }

    /// Apply substitutions to a type
    fn apply_substitutions(&self, ty: &Type, substitutions: &HashMap<usize, Type>) -> Type {
        match ty {
            Type::Var(var) => {
                if let Some(replacement) = substitutions.get(&var.id) {
                    replacement.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Array(inner) => Type::Array(Box::new(
                self.apply_substitutions(inner, substitutions),
            )),
            Type::Map(key, val) => Type::Map(
                Box::new(self.apply_substitutions(key, substitutions)),
                Box::new(self.apply_substitutions(val, substitutions)),
            ),
            Type::Tuple(types) => Type::Tuple(
                types
                    .iter()
                    .map(|t| self.apply_substitutions(t, substitutions))
                    .collect(),
            ),
            Type::Function {
                params,
                return_type,
                is_async,
            } => Type::Function {
                params: params
                    .iter()
                    .map(|t| self.apply_substitutions(t, substitutions))
                    .collect(),
                return_type: Box::new(self.apply_substitutions(return_type, substitutions)),
                is_async: *is_async,
            },
            _ => ty.clone(),
        }
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Type errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeError {
    /// Type mismatch
    Mismatch(Type, Type),
    /// Undefined type
    UndefinedType(String),
    /// Undefined variable
    UndefinedVariable(String),
    /// Arity mismatch
    ArityMismatch(usize, usize),
    /// Not a function
    NotFunction(Type),
    /// Not numeric
    NotNumeric(Type),
    /// Not comparable
    NotComparable(Type),
    /// Not callable
    NotCallable(Type),
    /// Not indexable
    NotIndexable(Type),
    /// Not iterable
    NotIterable(Type),
    /// Missing field
    MissingField(String, Type),
    /// Duplicate definition
    DuplicateDefinition(String),
    /// Infinite type
    InfiniteType(Type),
    /// Constraint error
    ConstraintError(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Mismatch(expected, found) => {
                write!(f, "Type mismatch: expected `{}`, found `{}`", expected, found)
            }
            TypeError::UndefinedType(name) => write!(f, "Undefined type: {}", name),
            TypeError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            TypeError::ArityMismatch(expected, found) => {
                write!(f, "Arity mismatch: expected {} arguments, found {}", expected, found)
            }
            TypeError::NotFunction(ty) => write!(f, "Not a function: {}", ty),
            TypeError::NotNumeric(ty) => write!(f, "Not numeric: {}", ty),
            TypeError::NotComparable(ty) => write!(f, "Not comparable: {}", ty),
            TypeError::NotCallable(ty) => write!(f, "Not callable: {}", ty),
            TypeError::NotIndexable(ty) => write!(f, "Not indexable: {}", ty),
            TypeError::NotIterable(ty) => write!(f, "Not iterable: {}", ty),
            TypeError::MissingField(field, ty) => {
                write!(f, "Missing field `{}` in type `{}`", field, ty)
            }
            TypeError::DuplicateDefinition(name) => {
                write!(f, "Duplicate definition: {}", name)
            }
            TypeError::InfiniteType(ty) => write!(f, "Infinite type: {}", ty),
            TypeError::ConstraintError(msg) => write!(f, "Constraint error: {}", msg),
        }
    }
}

impl std::error::Error for TypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int.to_string(), "Int");
        assert_eq!(Type::String.to_string(), "String");
        assert_eq!(
            Type::Array(Box::new(Type::Int)).to_string(),
            "[Int]"
        );
        assert_eq!(
            Type::Map(Box::new(Type::String), Box::new(Type::Int)).to_string(),
            "{String: Int}"
        );
    }

    #[test]
    fn test_type_properties() {
        assert!(Type::Int.is_primitive());
        assert!(Type::Int.is_numeric());
        assert!(!Type::String.is_numeric());
        assert!(Type::String.is_comparable());
    }

    #[test]
    fn test_type_assignability() {
        assert!(Type::Int.is_assignable_to(&Type::Int));
        assert!(Type::Int.is_assignable_to(&Type::Float));
        assert!(!Type::String.is_assignable_to(&Type::Int));
    }

    #[test]
    fn test_type_context() {
        let mut ctx = TypeContext::new();
        let var = ctx.new_type_var(Some("t".to_string()));
        ctx.add_variable("x".to_string(), var);
        assert!(ctx.get_variable("x").is_some());
    }

    #[test]
    fn test_unification() {
        let mut ctx = TypeContext::new();
        let var = ctx.new_type_var(None);
        ctx.add_constraint(TypeConstraint::Equal(var.clone(), Type::Int));

        let substitutions = ctx.solve().unwrap();
        assert_eq!(substitutions.get(&0), Some(&Type::Int));
    }
}
