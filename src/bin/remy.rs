extern crate clap;
use clap::Arg;
extern crate time;
#[macro_use]
extern crate slog;

extern crate ccp_remy;
extern crate portus;

use ccp_remy::Remy;
use portus::ipc::{BackendBuilder, Blocking};

fn make_args() -> Result<(ccp_remy::RemyConfig, String), String> {
    let matches = clap::App::new("CCP Remy")
        .version("0.2.0")
        .author("Deepti Raghavan <deeptir@mit.edu>")
        .about("Implementation of Remy Congestion Control")
        .arg(Arg::with_name("ipc")
             .long("ipc")
             .help("Sets the type of ipc to use: (netlink|unix)")
             .default_value("unix")
             .validator(portus::algs::ipc_valid))
        .arg(Arg::with_name("input_whiskers")
             .long("input_whiskers")
             .help("Protobuf file with remy whiskers located.")
             .default_value("/home/ubuntu/remy/tests/RemyCC-2014-100x.dna"))
        .get_matches();


    Ok((
        ccp_remy::RemyConfig {
            input_whiskers: String::from(matches.value_of("input_whiskers").unwrap()),
        },
        String::from(matches.value_of("ipc").unwrap()),
    ))
}

fn main() {
    let log = portus::algs::make_logger();
    let (cfg, ipc) = make_args()
        .map_err(|e| warn!(log, "bad argument"; "err" => ?e))
        .unwrap_or_default();

    info!(log, "starting CCP"; 
        "algorithm" => "BBR",
        "ipc" => ipc.clone(),
    );
    match ipc.as_str() {
        "unix" => {
            use portus::ipc::unix::Socket;
            let b = Socket::<Blocking>::new("in", "out")
                .map(|sk| BackendBuilder{sock: sk})
                .expect("ipc initialization");
            portus::run::<_, Remy<_>>(
                b,
                &portus::Config {
                    logger: Some(log),
                    config: cfg,
                }
            ).unwrap();
        }
        #[cfg(all(target_os = "linux"))]
        "netlink" => {
            use portus::ipc::netlink::Socket;
            let b = Socket::<Blocking>::new()
                .map(|sk| BackendBuilder{sock: sk})
                .expect("ipc initialization");
            portus::run::<_, Remy<_>>(
                b,
                &portus::Config {
                    logger: Some(log),
                    config: cfg,
                }
            ).unwrap();
        }
        _ => unreachable!(),
    }
            
}
