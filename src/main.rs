mod conf_reader;

use conf_reader::{read_sway_config, Config};

fn main() {
    read_sway_config("/home/tirth/Projects/gsoc-tasks/sway_config_override/files/config")
        .expect("Error in main");

    // let p = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/";
    // let p2 = Path::new(p).as_os_str().to_str().unwrap();
    // println!("{p2}");
    // let p2 = "/home/tirth/Projects/gsoc-tasks/sway_config_override/files/test_configs/dir_1/";

    // let c1 = std::path::Path::new(p2).exists();
    // println!("{c1}");

    // let t = fs::read_to_string(&p).unwrap();

    // println!("{t}");

    // let s = read_sway_config(&p).unwrap();

    // println!("{:?}", s);
}
