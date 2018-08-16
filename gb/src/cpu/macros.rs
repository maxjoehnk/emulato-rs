macro_rules! pc {
    ($gb:expr) => ($gb.register.pc += 1);
    ($gb:expr, 1) => ($gb.register.pc += 1);
    ($gb:expr, 2) => ($gb.register.pc += 2);
    ($gb:expr, 3) => ($gb.register.pc += 3);
}

macro_rules! cmd {
    ($cmd:expr) => (Some(Box::new($cmd)));
}

macro_rules! u8 {
    ($rom:expr) => ($rom[0]);
}

macro_rules! u16 {
    ($rom:expr) => (LittleEndian::read_u16(&$rom[0..2]));
}

macro_rules! i8 {
    ($rom:expr) => ($rom[0] as i8);
}

macro_rules! r8 {
    ($register:ident) => (Register8::$register);
}

macro_rules! rp {
    ($register:ident) => (RegisterPair::$register);
}