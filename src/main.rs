//Mods
mod sys_mem;
mod gen_utils;

//Uses
use sys_mem::SysMem;

fn main() {
    let memory = SysMem::new();
    let number = gen_utils::hex_string_to_int("F");
    let mut newnumber: u64 = 5;
    if number.is_some() {
        newnumber = number.unwrap();
    }
    println!("{}", newnumber.to_string());
}
