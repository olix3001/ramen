#[derive(Debug, Clone)]
pub enum RamenType {
    // Primitive types
    Unit,
    Integer(usize),

    // More complex types
    Callable(Box<CallableType>)
}

#[derive(Debug, Clone)]
pub struct CallableType {
    pub return_type: RamenType,
    pub parameter_types: Vec<RamenType>,
    pub is_vararg: bool
}

impl CallableType {
    pub fn new(return_type: RamenType, parameter_types: Vec<RamenType>) -> Self {
        Self {
            return_type,
            parameter_types,
            is_vararg: false
        }
    }
}