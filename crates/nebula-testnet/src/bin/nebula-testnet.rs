use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    let wants_json = args.iter().any(|arg| arg == "--json");
    let wants_readiness = args.iter().any(|arg| arg == "--mainnet-readiness");
    let wants_sample_attestation = args
        .iter()
        .any(|arg| arg == "--sample-deployment-attestation");
    let wants_sample_validator_set = args.iter().any(|arg| arg == "--sample-validator-set");
    let wants_sample_genesis_manifest = args.iter().any(|arg| arg == "--sample-genesis-manifest");
    let wants_build_genesis_manifest = args.iter().any(|arg| arg == "--build-genesis-manifest");
    let wants_verify_launch_package = args.iter().any(|arg| arg == "--verify-launch-package");
    let wants_sample_public_status = args.iter().any(|arg| arg == "--sample-public-status");
    let wants_sample_public_probe = args.iter().any(|arg| arg == "--sample-public-probe");

    if wants_sample_attestation {
        println!(
            "{}",
            nebula_testnet::sample_deployment_attestation_json_pretty()
        );
    } else if wants_sample_validator_set {
        println!("{}", nebula_testnet::sample_validator_set_json_pretty());
    } else if wants_sample_genesis_manifest {
        println!("{}", nebula_testnet::sample_genesis_manifest_json_pretty());
    } else if wants_sample_public_status {
        println!(
            "{}",
            nebula_testnet::sample_public_status_manifest_json_pretty()
        );
    } else if wants_sample_public_probe {
        println!("{}", nebula_testnet::sample_public_probe_json_pretty());
    } else if let Some(path) = arg_value(&args, "--verify-deployment-attestation") {
        verify_attestation(path, wants_json);
    } else if let Some(path) = arg_value(&args, "--verify-public-status") {
        verify_public_status(path, wants_json);
    } else if let Some(path) = arg_value(&args, "--verify-public-probe") {
        verify_public_probe(path, wants_json);
    } else if let Some(path) = arg_value(&args, "--verify-validator-set") {
        verify_validator_set(path, wants_json);
    } else if wants_build_genesis_manifest {
        build_genesis_manifest(&args, wants_json);
    } else if let Some(path) = arg_value(&args, "--verify-genesis-manifest") {
        verify_genesis_manifest(path, wants_json);
    } else if wants_verify_launch_package {
        verify_launch_package(&args, wants_json);
    } else if wants_json || wants_readiness {
        println!("{}", nebula_testnet::readiness_json_pretty());
    } else {
        println!("{}", nebula_testnet::readiness_summary());
    }
}

fn arg_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].as_str())
}

fn verify_public_status(path: &str, wants_json: bool) {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            print_public_status_error(wants_json, &[format!("failed to read {path}: {error}")]);
            process::exit(1);
        }
    };

    match nebula_testnet::verify_public_status_manifest_json(&input) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("public status report serializes")
                );
            } else {
                println!(
                    "Public status verified at {}.",
                    report.public_status_manifest_root
                );
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_public_status_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_public_status_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn verify_public_probe(path: &str, wants_json: bool) {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            print_public_probe_error(wants_json, &[format!("failed to read {path}: {error}")]);
            process::exit(1);
        }
    };

    match nebula_testnet::verify_public_probe_json(&input) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("public probe report serializes")
                );
            } else {
                println!("Public probe verified at {}.", report.public_probe_root);
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_public_probe_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_public_probe_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn verify_attestation(path: &str, wants_json: bool) {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            print_verification_error(wants_json, &[format!("failed to read {path}: {error}")]);
            process::exit(1);
        }
    };

    match nebula_testnet::verify_deployment_attestation_json(&input) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("verification report serializes")
                );
            } else {
                println!(
                    "Deployment attestation verified. Public launch gate can advance to {}.",
                    report.level
                );
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_verification_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_verification_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn verify_launch_package(args: &[String], wants_json: bool) {
    let Some(deployment_path) = arg_value(args, "--deployment-attestation") else {
        print_launch_package_error(
            wants_json,
            &["missing --deployment-attestation <path>".to_string()],
        );
        process::exit(1);
    };
    let Some(validator_set_path) = arg_value(args, "--validator-set") else {
        print_launch_package_error(wants_json, &["missing --validator-set <path>".to_string()]);
        process::exit(1);
    };
    let Some(genesis_path) = arg_value(args, "--genesis-manifest") else {
        print_launch_package_error(
            wants_json,
            &["missing --genesis-manifest <path>".to_string()],
        );
        process::exit(1);
    };

    let deployment_input = match fs::read_to_string(deployment_path) {
        Ok(input) => input,
        Err(error) => {
            print_launch_package_error(
                wants_json,
                &[format!("failed to read {deployment_path}: {error}")],
            );
            process::exit(1);
        }
    };
    let validator_set_input = match fs::read_to_string(validator_set_path) {
        Ok(input) => input,
        Err(error) => {
            print_launch_package_error(
                wants_json,
                &[format!("failed to read {validator_set_path}: {error}")],
            );
            process::exit(1);
        }
    };
    let genesis_input = match fs::read_to_string(genesis_path) {
        Ok(input) => input,
        Err(error) => {
            print_launch_package_error(
                wants_json,
                &[format!("failed to read {genesis_path}: {error}")],
            );
            process::exit(1);
        }
    };

    match nebula_testnet::verify_launch_package_jsons(
        &deployment_input,
        &validator_set_input,
        &genesis_input,
    ) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report)
                        .expect("launch package report serializes")
                );
            } else {
                println!("Launch package verified at {}.", report.genesis_root);
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_launch_package_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_launch_package_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn build_genesis_manifest(args: &[String], wants_json: bool) {
    let Some(deployment_path) = arg_value(args, "--deployment-attestation") else {
        print_genesis_manifest_error(
            wants_json,
            &["missing --deployment-attestation <path>".to_string()],
        );
        process::exit(1);
    };
    let Some(validator_set_path) = arg_value(args, "--validator-set") else {
        print_genesis_manifest_error(wants_json, &["missing --validator-set <path>".to_string()]);
        process::exit(1);
    };

    let deployment_input = match fs::read_to_string(deployment_path) {
        Ok(input) => input,
        Err(error) => {
            print_genesis_manifest_error(
                wants_json,
                &[format!("failed to read {deployment_path}: {error}")],
            );
            process::exit(1);
        }
    };
    let validator_set_input = match fs::read_to_string(validator_set_path) {
        Ok(input) => input,
        Err(error) => {
            print_genesis_manifest_error(
                wants_json,
                &[format!("failed to read {validator_set_path}: {error}")],
            );
            process::exit(1);
        }
    };

    match nebula_testnet::build_genesis_manifest_json_pretty(
        &deployment_input,
        &validator_set_input,
    ) {
        Ok(output) => println!("{output}"),
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_genesis_manifest_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_genesis_manifest_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn verify_genesis_manifest(path: &str, wants_json: bool) {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            print_genesis_manifest_error(wants_json, &[format!("failed to read {path}: {error}")]);
            process::exit(1);
        }
    };

    match nebula_testnet::verify_genesis_manifest_json(&input) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report)
                        .expect("genesis manifest report serializes")
                );
            } else {
                println!("Genesis manifest verified at {}.", report.genesis_root);
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_genesis_manifest_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_genesis_manifest_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn verify_validator_set(path: &str, wants_json: bool) {
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            print_validator_set_error(wants_json, &[format!("failed to read {path}: {error}")]);
            process::exit(1);
        }
    };

    match nebula_testnet::verify_validator_set_json(&input) {
        Ok(report) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("validator set report serializes")
                );
            } else {
                println!("Validator set verified at {}.", report.validator_set_root);
            }
        }
        Err(nebula_testnet::AttestationError::MalformedJson(error)) => {
            print_validator_set_error(wants_json, &[error]);
            process::exit(1);
        }
        Err(nebula_testnet::AttestationError::Invalid(errors)) => {
            print_validator_set_error(wants_json, &errors);
            process::exit(1);
        }
    }
}

fn print_verification_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "public_launch_ready": false,
                "level": "public-launch-attestation-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Deployment attestation rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_public_status_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "public_status_ready": false,
                "level": "public-status-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Public status rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_public_probe_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "public_probe_ready": false,
                "level": "public-probe-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Public probe rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_validator_set_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "validator_set_ready": false,
                "level": "validator-set-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Validator set rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_genesis_manifest_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "genesis_ready": false,
                "level": "genesis-manifest-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Genesis manifest rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_launch_package_error(wants_json: bool, errors: &[String]) {
    if wants_json {
        println!(
            "{}",
            serde_json::json!({
                "launch_package_ready": false,
                "level": "launch-package-rejected",
                "errors": errors,
            })
        );
    } else {
        eprintln!("Launch package rejected:");
        for error in errors {
            eprintln!("- {error}");
        }
    }
}

fn print_help() {
    println!(
        "nebula-testnet\n\nUSAGE:\n    nebula-testnet [--mainnet-readiness] [--json]\n    nebula-testnet --sample-public-status\n    nebula-testnet --verify-public-status <path> [--json]\n    nebula-testnet --sample-public-probe\n    nebula-testnet --verify-public-probe <path> [--json]\n    nebula-testnet --sample-deployment-attestation\n    nebula-testnet --verify-deployment-attestation <path> [--json]\n    nebula-testnet --sample-validator-set\n    nebula-testnet --verify-validator-set <path> [--json]\n    nebula-testnet --sample-genesis-manifest\n    nebula-testnet --build-genesis-manifest --deployment-attestation <path> --validator-set <path>\n    nebula-testnet --verify-genesis-manifest <path> [--json]\n    nebula-testnet --verify-launch-package --deployment-attestation <path> --validator-set <path> --genesis-manifest <path> [--json]\n\nOPTIONS:\n    --mainnet-readiness              Emit the public launch readiness contract\n    --sample-public-status           Emit a public status manifest sample\n    --verify-public-status           Verify a public status manifest file\n    --sample-public-probe            Emit a public probe sample\n    --verify-public-probe            Verify a public probe file\n    --sample-deployment-attestation  Emit a fillable deployment attestation sample\n    --verify-deployment-attestation  Verify a deployment attestation file\n    --sample-validator-set           Emit a fillable validator-set manifest sample\n    --verify-validator-set           Verify a validator-set manifest file\n    --sample-genesis-manifest        Emit a sample genesis manifest built from samples\n    --build-genesis-manifest         Build genesis manifest from attestation and validator set\n    --deployment-attestation         Deployment attestation input for genesis build/package verification\n    --validator-set                  Validator-set input for genesis build/package verification\n    --genesis-manifest               Genesis manifest input for launch package verification\n    --verify-genesis-manifest        Verify a genesis manifest file\n    --verify-launch-package          Verify deployment, validator set, and genesis agree\n    --json                           Emit JSON output\n    -h, --help                       Show this help"
    );
}
