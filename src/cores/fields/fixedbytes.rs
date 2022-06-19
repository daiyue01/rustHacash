
// create FixedBytes macro
macro_rules! create_fixedbytes_struct_and_impl{
    ($tip:expr, $name:ident, $size:expr) => (

        
#[derive(PartialEq)]
pub struct $name {
    bytes: [u8; $size],
}

impl Clone for $name {
    fn clone(&self) -> $name {
        $name{
            bytes: self.bytes.clone(),
        }
    }
}

impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        if $size != self.bytes.len() {
            panic!("{}.serialize size not match.", $tip)
        }
        self.bytes.to_vec()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let seek2 = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.try_into().unwrap();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        <$name>::size()
    }

    fn describe(&self) -> String {
        format!("\"{}\"", hex::encode(self.bytes))
    }

} 


impl $name {

    const fn size() -> usize {
        $size
    }

    pub fn new() -> $name {
        $name{
            bytes: [0u8; $size],
        }
    }

    pub fn value(&self) -> [u8; $size] {
        self.bytes.clone()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).unwrap()
    }

    pub fn from( v: [u8; $size] ) -> $name {
        $name{
            bytes: v,
        }
    }

    pub fn clone(&self) -> $name {
        $name{
            bytes: self.bytes.clone(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::from([0u8; $size])});

}




    )
}



// create 
create_fixedbytes_struct_and_impl!("Fixedbytes1 ", Fixedbytes1 ,  1usize);
create_fixedbytes_struct_and_impl!("Fixedbytes2 ", Fixedbytes2 ,  2usize);
create_fixedbytes_struct_and_impl!("Fixedbytes3 ", Fixedbytes3 ,  3usize);
create_fixedbytes_struct_and_impl!("Fixedbytes4 ", Fixedbytes4 ,  4usize);
create_fixedbytes_struct_and_impl!("Fixedbytes5 ", Fixedbytes5 ,  5usize);
create_fixedbytes_struct_and_impl!("Fixedbytes6 ", Fixedbytes6 ,  6usize);
create_fixedbytes_struct_and_impl!("Fixedbytes8 ", Fixedbytes8 ,  8usize);
create_fixedbytes_struct_and_impl!("Fixedbytes10", Fixedbytes10, 10usize);
create_fixedbytes_struct_and_impl!("Fixedbytes12", Fixedbytes12, 12usize);
create_fixedbytes_struct_and_impl!("Fixedbytes14", Fixedbytes14, 14usize);
create_fixedbytes_struct_and_impl!("Fixedbytes15", Fixedbytes15, 15usize);
create_fixedbytes_struct_and_impl!("Fixedbytes16", Fixedbytes16, 16usize);
create_fixedbytes_struct_and_impl!("Fixedbytes17", Fixedbytes17, 17usize);
create_fixedbytes_struct_and_impl!("Fixedbytes18", Fixedbytes18, 18usize);
create_fixedbytes_struct_and_impl!("Fixedbytes21", Fixedbytes21, 21usize);
create_fixedbytes_struct_and_impl!("Fixedbytes24", Fixedbytes24, 24usize);
create_fixedbytes_struct_and_impl!("Fixedbytes32", Fixedbytes32, 32usize);
create_fixedbytes_struct_and_impl!("Fixedbytes33", Fixedbytes33, 33usize);
create_fixedbytes_struct_and_impl!("Fixedbytes64", Fixedbytes64, 64usize);
