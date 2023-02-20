mod logic;

use clap::{Arg, Command};
use logic::OutputFormat;

fn main() {
    let matches = Command::new("huawei_solar")
        .about("Huawei Solar Inverter CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("fildaw")
        .subcommand(
            Command::new("query")
                .about("Query the inverter")
                .arg(
                    Arg::new("ip_port")
                        .help("ip:[port] of the inverter (or S-Dongle if Modbus TCP is enabled, default port when no specified: 502)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("query_params")
                        .help("Query parameters (comma separated, use 'all' to query all params), allowed: model_name,serial_number,input_power,grid_voltage,line_voltage_a_b,line_voltage_b_c,line_voltage_c_a,phase_a_voltage,phase_b_voltage,phase_c_voltage,phase_a_current,phase_b_current,phase_c_current,day_active_power_peak,active_power,reactive_power,power_factor,grid_frequency,efficiency,internal_temperature,insulation_resistance,device_status,startup_time,shutdown_time,accumulated_yield_energy,daily_yield_energy")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("output_destination")
                        .help("Output destination filename ( - for stdout )")
                        .required(true)
                        .index(3),
                )
                .arg(
                    Arg::new("output_format")
                        .short('f')
                        .long("output_format")
                        .help("Output format (json, pretty_print)"),
                )
                .arg(
                    Arg::new("slave_id")
                        .long("slave_id")
                        .help("Modbus slave id (default: 0) (specify 1 for connecting through S-Dongle)"),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("query", query_matches)) => {
            let ip_port = query_matches.get_one::<String>("ip_port").unwrap();
            let query_params = query_matches.get_one::<String>("query_params").unwrap();
            let output_destination = query_matches.get_one::<String>("output_destination").unwrap();
            let chosen_output_format = if let Some(output_format)= query_matches.get_one::<String>("output_format") {
                match output_format.as_str() {
                    "json" => {println!("Using json as output format"); OutputFormat::Json},
                    "pretty_print" => {println!("Using pretty_print as output format"); OutputFormat::PrettyPrint},
                    _ => {println!("Unknown format: {}. Using default (json) as output format", output_format); OutputFormat::Json},
                }
            } else {
                println!("Using default (json) as output format");
                OutputFormat::Json
            };
            let slave_id = if let Some(slave_id) = query_matches.get_one::<String>("slave_id") {
                println!("Using slave id: {}", slave_id);
                slave_id.parse::<u8>().expect("Bad slave id!")
            } else {
                println!("Using default slave id: 0");
                0
            };
            let ip = ip_port.split(":").next().unwrap();
            let port = ip_port.split(":").nth(1).unwrap_or("502").parse::<u16>().expect("Bad port!");
            logic::query(ip, port, slave_id, &query_params, chosen_output_format, &output_destination);
        },
        _ => unreachable!(),
    }
}
