macro_rules! pc {
    ($gb:expr) => ($gb.register.pc += 1);
    ($gb:expr, 1) => ($gb.register.pc += 1);
    ($gb:expr, 2) => ($gb.register.pc += 2);
    ($gb:expr, 3) => ($gb.register.pc += 3);
}