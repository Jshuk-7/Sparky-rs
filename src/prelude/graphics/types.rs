///  Shapes to specify what kind of primitive to render
pub enum Primitive {
    Triangle,
    Line,
    Point,
}

impl Primitive {
    pub fn prim_type(&self) -> u32 {
        match self {
            Primitive::Triangle => gl::TRIANGLES,
            Primitive::Line => gl::LINES,
            Primitive::Point => gl::POINTS,
        }
    }
}

/// Gpu buffer types
pub enum BufferType {
    Vertex,
    Index,
}

impl BufferType {
    pub fn buf_type(&self) -> u32 {
        match self {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
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
    Float64,
}

impl BufferDataType {
    pub fn data_type(&self) -> u32 {
        use BufferDataType as Dt;

        match self {
            Dt::UByte => gl::UNSIGNED_BYTE,
            Dt::Uint16 => gl::UNSIGNED_SHORT,
            Dt::Uint32 => gl::UNSIGNED_INT,
            Dt::Byte => gl::BYTE,
            Dt::Int16 => gl::SHORT,
            Dt::Int32 => gl::INT,
            Dt::Float32 => gl::FLOAT,
            Dt::Float64 => gl::DOUBLE,
        }
    }
}

/// Gpu buffer usage type, choosing the incorrect type will result in a decrease in performance
pub enum BufferUsage {
    Default,
    Single,
    Multi,
}

impl BufferUsage {
    pub fn buf_usage(&self) -> u32 {
        match self {
            BufferUsage::Default => gl::STATIC_DRAW,
            BufferUsage::Single => gl::STATIC_DRAW,
            BufferUsage::Multi => gl::DYNAMIC_DRAW,
        }
    }
}

pub enum Color {
    Red,
    LightRed,
    Orange,
    Yellow,
    Green,
    LightGreen,
    LightBlue,
    Blue,
    Pink,
    Purple,
    Gray,
    Black,
    White,
}

impl Color {
    pub fn color(&self) -> [f32; 3] {
        use Color as C;

        match self {
            C::Red => [1.0, 0.0, 0.0],
            C::LightRed => [0.8, 0.2, 0.2],
            C::Orange => [1.0, 0.64, 0.0],
            C::Yellow => [1.0, 1.0, 0.0],
            C::Green => [0.2, 0.8, 0.2],
            C::LightGreen => [0.0, 1.0, 0.0],
            C::LightBlue => [0.2, 0.2, 0.8],
            C::Blue => [0.0, 0.0, 1.0],
            C::Pink => [1.0, 0.75, 0.80],
            C::Purple => [0.45, 0.35, 0.75],
            C::Gray => [0.2, 0.2, 0.2],
            C::Black => [0.0, 0.0, 0.0],
            C::White => [1.0, 1.0, 1.0],
        }
    }
}

#[derive(PartialEq)]
pub enum ShaderType {
    Vert,
    Frag,
}

impl ShaderType {
    pub fn shader_type(&self) -> u32 {
        match self {
            ShaderType::Vert => gl::VERTEX_SHADER,
            ShaderType::Frag => gl::FRAGMENT_SHADER,
        }
    }
}