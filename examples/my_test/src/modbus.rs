
pub fn test() {
    
    use tokio_modbus::prelude::*;
    let socket_addr = "153.160.2.103:502".parse().unwrap();
    let mut ctx = sync::tcp::connect(socket_addr).unwrap();
    let buff = ctx.read_input_registers(0x1000, 7).unwrap();
    println!("Response is '{:?}'", buff);
}