use std::{env, process};

use nebula_l2_rs::{
    devnet_summary_line, DevnetRunConfig, DevnetRunner, NebulaRuntimeProfile, DEVNET_RUNNER_VERSION,
};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Devnet(CliDevnetOptions),
    Testnet(CliDevnetOptions),
    Help,
    Version,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CliDevnetOptions {
    blocks: u64,
    operator_label: String,
    profile_name: String,
    json: bool,
    enable_rpc: bool,
    enable_p2p: bool,
    enable_relayer: bool,
    enable_workload: bool,
    workload_batch_size: u64,
    block_time_ms: u64,
}

impl Default for CliDevnetOptions {
    fn default() -> Self {
        Self {
            blocks: 6,
            operator_label: "devnet-operator".to_string(),
            profile_name: "devnet".to_string(),
            json: false,
            enable_rpc: true,
            enable_p2p: true,
            enable_relayer: true,
            enable_workload: true,
            workload_batch_size: 24,
            block_time_ms: nebula_l2_rs::TARGET_BLOCK_MS,
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("nebula-daemon error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    match parse_args(env::args().skip(1).collect::<Vec<_>>())? {
        Command::Devnet(options) => run_devnet(options),
        Command::Testnet(options) => run_devnet(options),
        Command::Help => {
            print_help();
            Ok(())
        }
        Command::Version => {
            println!("{DEVNET_RUNNER_VERSION}");
            Ok(())
        }
    }
}

fn run_devnet(options: CliDevnetOptions) -> Result<(), String> {
    let config = DevnetRunConfig::default()
        .with_blocks(options.blocks)
        .with_operator_label(options.operator_label)
        .with_block_time_ms(options.block_time_ms)
        .with_rpc(options.enable_rpc)
        .with_p2p(options.enable_p2p)
        .with_relayer(options.enable_relayer)
        .with_workload(options.enable_workload)
        .with_workload_batch_size(options.workload_batch_size);
    let runtime_profile = NebulaRuntimeProfile::from_profile_name(&options.profile_name)?
        .with_operator_label(config.operator_label.clone())
        .with_block_time_ms(config.block_time_ms);
    let mut runner = DevnetRunner::from_runtime_profile(config, runtime_profile)?;
    let summary = runner.run_to_completion()?;
    if options.json {
        let output = json!({
            "summary": summary.public_record(),
            "summary_root": summary.summary_root(),
            "runner_root": runner.runner_root(),
            "daemon_root": runner.daemon.daemon_root(),
            "rpc_state_root": runner.rpc.state_root(),
            "p2p_overlay_root": runner.p2p.overlay_root(),
            "relayer_state_root": runner.relayer.state_root(),
            "storage_manifest_root": runner.daemon.storage.manifest_root(),
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output)
                .map_err(|error| format!("failed to encode devnet output: {error}"))?
        );
    } else {
        println!("{}", devnet_summary_line(&summary));
        println!("summary_root={}", summary.summary_root());
        println!("runner_root={}", runner.runner_root());
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Command, String> {
    if args.is_empty() {
        return Ok(Command::Help);
    }
    match args[0].as_str() {
        "devnet" => parse_devnet_args(&args[1..]).map(Command::Devnet),
        "testnet" => {
            let mut options = parse_devnet_args(&args[1..])?;
            if options.profile_name == CliDevnetOptions::default().profile_name {
                options.profile_name = "testnet".to_string();
            }
            Ok(Command::Testnet(options))
        }
        "help" | "--help" | "-h" => Ok(Command::Help),
        "version" | "--version" | "-V" => Ok(Command::Version),
        other => Err(format!("unknown command '{other}'")),
    }
}

fn parse_devnet_args(args: &[String]) -> Result<CliDevnetOptions, String> {
    let mut options = CliDevnetOptions::default();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--blocks" | "-b" => {
                index += 1;
                options.blocks = parse_u64_arg(args, index, "--blocks")?;
            }
            "--operator" | "--operator-label" => {
                index += 1;
                options.operator_label = parse_string_arg(args, index, "--operator")?;
            }
            "--profile" => {
                index += 1;
                options.profile_name = parse_string_arg(args, index, "--profile")?;
            }
            "--block-time-ms" => {
                index += 1;
                options.block_time_ms = parse_u64_arg(args, index, "--block-time-ms")?.max(1);
            }
            "--json" => {
                options.json = true;
            }
            "--no-rpc" => {
                options.enable_rpc = false;
            }
            "--no-p2p" => {
                options.enable_p2p = false;
            }
            "--no-relayer" => {
                options.enable_relayer = false;
            }
            "--no-workload" => {
                options.enable_workload = false;
            }
            "--workload-batch-size" => {
                index += 1;
                options.workload_batch_size =
                    parse_u64_arg(args, index, "--workload-batch-size")?.max(1);
            }
            "--all" => {
                options.enable_rpc = true;
                options.enable_p2p = true;
                options.enable_relayer = true;
                options.enable_workload = true;
            }
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            other => return Err(format!("unknown devnet option '{other}'")),
        }
        index += 1;
    }
    Ok(options)
}

fn parse_u64_arg(args: &[String], index: usize, flag: &str) -> Result<u64, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("{flag} requires a value"))?;
    value
        .parse::<u64>()
        .map_err(|error| format!("{flag} value '{value}' is not a positive integer: {error}"))
}

fn parse_string_arg(args: &[String], index: usize, flag: &str) -> Result<String, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("{flag} requires a value"))?;
    if value.is_empty() {
        return Err(format!("{flag} cannot be empty"));
    }
    Ok(value.clone())
}

fn print_help() {
    println!(
        "nebula-daemon {version}

USAGE:
    nebula-daemon devnet [--blocks N] [--profile NAME] [--operator LABEL] [--json]
    nebula-daemon testnet [--blocks N] [--operator LABEL] [--json]

COMMANDS:
    devnet      Run a deterministic in-process Monero L2 devnet loop
    testnet     Run the bridge testnet profile against committed Monero stagenet surfaces
    version     Print the devnet runner version
    help        Print this help

OPTIONS:
    -b, --blocks N          Number of blocks to produce
        --profile NAME      Runtime profile: devnet, local_fast, private_bridge, testnet, archive_validator
        --operator LABEL    Operator label used for deterministic devnet keys
        --block-time-ms N   Deterministic block time used for receipts
        --json              Print machine-readable roots and summary
        --no-rpc            Disable RPC activity in the loop
        --no-p2p            Disable P2P sync/gossip activity in the loop
        --no-relayer        Disable Monero relayer anchor activity in the loop
        --no-workload       Disable workload transaction admissions
        --workload-batch-size N
                            Number of workload intents to admit per block
        --all               Enable RPC, P2P, and relayer activity",
        version = DEVNET_RUNNER_VERSION
    );
}
