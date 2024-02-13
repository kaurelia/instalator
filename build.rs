#[macro_use]
extern crate dotenv_codegen;

use colorful::{Color, Colorful};
use dotenv::dotenv;
use dotenv_build::output;

fn main() {
    dotenv().ok();
    output(dotenv_build::Config::default()).unwrap();
    let allowed_values = vec![
        "zs_gokart125",
        "rs_bydgoszcz_rotax",
        "rs_torun_rotax",
        "rs_autodrom_slomczyn_rotax",
    ];
    if !allowed_values.contains(&dotenv!("FOLDER_NAME")) {
        panic!(
            "{}",
            "Not allowed value in FOLDER_NAME environment variable. Please check .env file"
                .color(Color::Red)
        );
    }
}
