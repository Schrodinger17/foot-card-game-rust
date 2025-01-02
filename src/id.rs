use uuid::Uuid;

pub struct Id;

macro_rules! IdGenerator {
    ($func:ident, $type:ty) => {
        #[allow(unused)]
        impl Id {
            pub fn $func() -> $type {
                Uuid::new_v4().as_u128() as $type
            }
        }
    };
}

IdGenerator!(new_usize, usize);
IdGenerator!(new_u64, u64);
IdGenerator!(new_u32, u32);
IdGenerator!(new_u16, u16);
IdGenerator!(new_u8, u8);
IdGenerator!(new_i64, i64);
IdGenerator!(new_i32, i32);
IdGenerator!(new_i16, i16);
IdGenerator!(new_i8, i8);
