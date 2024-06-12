mod manager;
use std::env;
use std::result::Result;

fn print_cmd_debug() {
    println!("--------------------------------------------------------");
    println!("Use the following command format: <image_name> <command>");
    println!("");
    println!("<image_type> to see supported image functions (e.g. png)");
    println!("--------------------------------------------------------");
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_cmd_debug();
        return Err("Please, enter a valid command");
    }

    let image_type: String = manager::get_image_type(&args[1]);
    let image_name: String = args[1].clone();
    let op: String = args[2].clone();

    match image_type.as_str() {
        ".jpeg" => println!("jpeg image"),
        ".png" => println!("png image"),
        _ => println!("Please, enter a compatible image file"),
    };

    // let _ = match op.as_str() {
    //     "gray" => convert_to_gray_png(&image_name),
    //     "gaussian_blur" => {
    //         let parameter: String = args[3].clone();
    //         gaussian_blur_png(&image_name, parameter.parse::<i32>().unwrap())
    //     }
    //     _ => Err("Please, enter a valid function"),
    // };

    Ok(())
}
