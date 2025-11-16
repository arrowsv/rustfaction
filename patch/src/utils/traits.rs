pub trait ToBytes: Sized {
    fn to_le_bytes(a: Self) -> Vec<u8>;
}

impl ToBytes for u16 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for u32 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for u64 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for i8 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for i16 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for i32 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl ToBytes for i64 {
    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}