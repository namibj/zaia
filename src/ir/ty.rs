// Allow for constraints to be defined across types for things like flow typing and optimization.
pub enum Ty {
    Nil,
    Bool,
    Int,
    Float,
    String,
    Function,
    Table,
    Foreign,
    Union,
    Any,
}
