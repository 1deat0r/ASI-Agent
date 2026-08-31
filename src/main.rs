use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use asi_agent::{
    BloodlineLedger, EffectClass, HarnessRegistry, IsolationMode, SkillInspector, SovereignRuntime,
    TaskEnvelope, create_checkpoint, generate_keypair, sign_genome, verify_checkpoint,
    verify_genome,
};
use clap::{Args, Parser, Subcommand};
use serde::Serialize;

#[derive(Debug, Parser)]
#[command(
    name = "asi",
    version,
    about = "ASI Agent sovereign evolutionary meta-harness nucleus",
    long_about = "Absorb. Recraft. Evolve. Existing harnesses remain untrusted workers beneath one sovereign policy and Bloodline."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Detect known harnesses and report their declared containment profiles.
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Produce a redacted, non-executing invocation plan.
    Plan(TaskArguments),
    /// Plan by default; execute only when --execute is explicitly supplied.
    Run {
        #[command(flatten)]
        task: TaskArguments,
        #[arg(long)]
        execute: bool,
        /// Digest emitted by a separately inspected plan.
        #[arg(long)]
        approved_plan_sha256: Option<String>,
        #[arg(long, default_value = ".asi/bloodline.jsonl")]
        ledger: PathBuf,
        /// Signed harness genome that must match current executable state.
        #[arg(long, default_value = ".asi/genome/signed.json")]
        genome: PathBuf,
        /// Explicitly pinned public key used to verify --genome.
        #[arg(long, default_value = ".asi/keys/lineage-public.json")]
        genome_public_key: PathBuf,
    },
    /// Inspect the integrity of the Bloodline ledger.
    Ledger {
        #[command(subcommand)]
        command: LedgerCommand,
    },
    /// Generate local lineage-signing keys without exposing private material.
    Key {
        #[command(subcommand)]
        command: KeyCommand,
    },
    /// Sign or verify the complete local harness genome.
    Genome {
        #[command(subcommand)]
        command: GenomeCommand,
    },
    /// Inspect or quarantine a third-party skill without executing it.
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
}

#[derive(Clone, Debug, Args)]
struct TaskArguments {
    #[arg(long, default_value = "auto")]
    harness: String,
    #[arg(long)]
    task: String,
    #[arg(long, value_enum, default_value_t = EffectClass::None)]
    effect: EffectClass,
    #[arg(long, default_value = ".")]
    cwd: PathBuf,
    #[arg(long, default_value_t = 120)]
    timeout: u64,
    #[arg(long, value_enum, default_value_t = IsolationMode::Required)]
    isolation: IsolationMode,
    /// Required second signal when deliberately disabling OS subprocess isolation.
    #[arg(long)]
    acknowledge_unsafe_subprocess: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum LedgerCommand {
    Verify {
        #[arg(long, default_value = ".asi/bloodline.jsonl")]
        path: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Sign the current terminal state and byte digest of a valid Bloodline.
    Checkpoint {
        #[arg(long, default_value = ".asi/bloodline.jsonl")]
        path: PathBuf,
        #[arg(long, default_value = ".asi/keys/lineage-private.json")]
        private_key: PathBuf,
        #[arg(long, default_value = ".asi/checkpoints/bloodline.json")]
        output: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Verify a signed checkpoint against a ledger and an explicitly pinned key.
    VerifyCheckpoint {
        #[arg(long, default_value = ".asi/bloodline.jsonl")]
        path: PathBuf,
        #[arg(long, default_value = ".asi/checkpoints/bloodline.json")]
        checkpoint: PathBuf,
        #[arg(long, default_value = ".asi/keys/lineage-public.json")]
        public_key: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum KeyCommand {
    Generate {
        #[arg(long, default_value = ".asi/keys/lineage-private.json")]
        private_key: PathBuf,
        #[arg(long, default_value = ".asi/keys/lineage-public.json")]
        public_key: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum GenomeCommand {
    /// Capture descriptors and executable fingerprints, then sign the snapshot.
    Sign {
        #[arg(long, default_value = ".asi/keys/lineage-private.json")]
        private_key: PathBuf,
        #[arg(long, default_value = ".asi/genome/signed.json")]
        output: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Verify both the signature and the currently detected harness state.
    Verify {
        #[arg(long, default_value = ".asi/genome/signed.json")]
        genome: PathBuf,
        #[arg(long, default_value = ".asi/keys/lineage-public.json")]
        public_key: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Subcommand)]
enum SkillCommand {
    Inspect {
        path: PathBuf,
        #[arg(long)]
        json: bool,
    },
    Assimilate {
        path: PathBuf,
        #[arg(long, default_value = ".asi/crypt")]
        vault: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Serialize)]
struct PlannedResponse<T: Serialize> {
    schema_version: &'static str,
    executed: bool,
    policy: asi_agent::PolicyDecision,
    plan: T,
}

#[tokio::main]
async fn main() -> ExitCode {
    match run(Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Doctor { json } => doctor(json),
        Command::Plan(arguments) => plan(arguments),
        Command::Run {
            task,
            execute,
            approved_plan_sha256,
            ledger,
            genome,
            genome_public_key,
        } => {
            run_task(
                task,
                execute,
                approved_plan_sha256,
                ledger,
                genome,
                genome_public_key,
            )
            .await
        }
        Command::Ledger { command } => match command {
            LedgerCommand::Verify { path, json } => {
                let verification = BloodlineLedger::verify(&path)?;
                print_value(&verification, json)
            }
            LedgerCommand::Checkpoint {
                path,
                private_key,
                output,
                json,
            } => {
                let checkpoint = create_checkpoint(path, private_key, output)?;
                print_value(&checkpoint, json)
            }
            LedgerCommand::VerifyCheckpoint {
                path,
                checkpoint,
                public_key,
                json,
            } => {
                let verification = verify_checkpoint(path, checkpoint, public_key)?;
                print_value(&verification, json)
            }
        },
        Command::Key { command } => match command {
            KeyCommand::Generate {
                private_key,
                public_key,
                json,
            } => {
                let outcome = generate_keypair(private_key, public_key)?;
                print_value(&outcome, json)
            }
        },
        Command::Genome { command } => match command {
            GenomeCommand::Sign {
                private_key,
                output,
                json,
            } => {
                let genome = sign_genome(private_key, output)?;
                print_value(&genome, json)
            }
            GenomeCommand::Verify {
                genome,
                public_key,
                json,
            } => {
                let verification = verify_genome(genome, public_key)?;
                print_value(&verification, json)
            }
        },
        Command::Skill { command } => match command {
            SkillCommand::Inspect { path, json } => {
                let report = SkillInspector.inspect(path)?;
                print_value(&report, json)
            }
            SkillCommand::Assimilate { path, vault, json } => {
                let outcome = SkillInspector.assimilate(path, vault)?;
                print_value(&outcome, json)
            }
        },
    }
}

fn doctor(json: bool) -> Result<()> {
    let harnesses = HarnessRegistry.discover();
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&harnesses).context("cannot encode doctor report")?
        );
    } else {
        println!("ASI Agent harness genome");
        println!("========================");
        for harness in harnesses {
            let status = if harness.installed {
                "detected"
            } else {
                "missing"
            };
            println!(
                "{:<20} {:<9} {}",
                harness.descriptor.id,
                status,
                harness.version.as_deref().unwrap_or("version unavailable")
            );
            println!(
                "  authority={} effects={}",
                harness.descriptor.authority_owner,
                harness
                    .descriptor
                    .supported_effects
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }
    Ok(())
}

fn plan(arguments: TaskArguments) -> Result<()> {
    let task = build_task(&arguments)?;
    let prepared = SovereignRuntime::default().prepare_with_isolation(
        task,
        &arguments.harness,
        arguments.isolation,
        arguments.acknowledge_unsafe_subprocess,
    )?;
    let response = PlannedResponse {
        schema_version: "asi.planned-response.v0.1",
        executed: false,
        policy: prepared.policy.clone(),
        plan: prepared.public_plan(),
    };
    print_value(&response, arguments.json)
}

async fn run_task(
    arguments: TaskArguments,
    execute: bool,
    approved_plan_sha256: Option<String>,
    ledger: PathBuf,
    genome: PathBuf,
    genome_public_key: PathBuf,
) -> Result<()> {
    let task = build_task(&arguments)?;
    let runtime = SovereignRuntime::default();
    let prepared = runtime.prepare_with_isolation(
        task,
        &arguments.harness,
        arguments.isolation,
        arguments.acknowledge_unsafe_subprocess,
    )?;
    if !execute {
        let response = PlannedResponse {
            schema_version: "asi.planned-response.v0.1",
            executed: false,
            policy: prepared.policy.clone(),
            plan: prepared.public_plan(),
        };
        print_value(&response, arguments.json)?;
        if !arguments.json {
            println!(
                "\nNot executed. Re-run with --execute --approved-plan-sha256 {} after inspecting the plan.",
                response.plan.plan_sha256
            );
        }
        return Ok(());
    }

    let approved_plan_sha256 = approved_plan_sha256
        .context("--execute requires --approved-plan-sha256 from a separately inspected plan")?;
    let verified_genome = verify_genome(&genome, &genome_public_key)
        .context("execution requires a signed genome matching the pinned key and current state")?;
    let outcome = runtime
        .execute(prepared, ledger, &approved_plan_sha256, &verified_genome)
        .await?;
    print_value(&outcome, arguments.json)
}

fn build_task(arguments: &TaskArguments) -> Result<TaskEnvelope> {
    TaskEnvelope::new(
        arguments.task.clone(),
        arguments.effect,
        &arguments.cwd,
        arguments.timeout,
    )
}

fn print_value<T: Serialize>(value: &T, json: bool) -> Result<()> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(value).context("cannot encode JSON output")?
        );
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(value).context("cannot encode output")?
        );
    }
    Ok(())
}
