mod reverse_print;
mod reverse_list;
mod chain_copy;


pub fn main() {
    let default = 1;

    match default {
        1 => reverse_print::main(),
        2 => reverse_list::main(),
        _ => {}
    }
}

