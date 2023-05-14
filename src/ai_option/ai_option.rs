/// THIS IS THE AiOption ENUM
/// it can be used to replace the `Option`
/// it does not the same
/// it only does the same
/// T is a generic from any type
pub enum AiOption<T> {
    // Stores the Struct, the type is specified by the T generic
    Some(T),
    // this is a a enum type that represents that there is no
    // value stored in this enum
    None,
}
