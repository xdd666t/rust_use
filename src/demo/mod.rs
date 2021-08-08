mod call_back;
mod struct_use;
mod option_use;

pub fn main() {
    let default = 4;


    match default {
        //闭包写法
        1 => {
            invoke(call_back::main);
            call_back::main_set();
        }
        2 => invoke(struct_use::main),
        3 => invoke(option_use::main),
        _ => {}
    }
}

fn invoke(method: fn()) {
    println!();
    println!("测试demo: 打印开始");
    println!("------------------------------------------");
    method();
    println!("------------------------------------------");
    println!("测试demo: 打印结束");
    println!();
}