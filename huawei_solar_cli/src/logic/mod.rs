use huawei_solar_rs::HuaweiSolar;

use std::fs::File;
use std::io::Write;

pub enum OutputFormat {
    Json,
    PrettyPrint
}

mod params_parse;
mod pretty_print;

pub fn query(ip: &str, port: u16, slave_id: u8, query_params: &str, output_format: OutputFormat, 
    output_path: &str) 
{
    let mut inverter = HuaweiSolar::new_connection(ip, port, slave_id).unwrap();
    let map = params_parse::parse_to_map(&mut inverter, query_params);

    let output: String = match output_format {
        OutputFormat::Json => {
            serde_json::to_string(&map).unwrap()
        },
        OutputFormat::PrettyPrint => {
            pretty_print::print(&map)
        }
    };
    if output_path == "-" {
        println!("");
        println!("{}", output);
    } else {
        println!("Writing to {}", output_path);
        let mut file = File::create(output_path).unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }
}