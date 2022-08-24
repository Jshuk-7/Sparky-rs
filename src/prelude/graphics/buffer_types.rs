/// Gpu buffer types
pub enum BufferType {
    Vertex,
    Index,
}

/// Gpu buffer data types
pub enum BufferDataType {
    UByte,
    Uint16,
    Uint32,
    Byte,
    Int16,
    Int32,
    Float32,
}

/// Gpu buffer usage type, choosing the incorrect type will result in a decrease in performance 
pub enum BufferUsage {
    Default,
    Single,
    Multi,
}
