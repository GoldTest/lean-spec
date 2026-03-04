use colored::Colorize;
use leanspec_core::sessions::RunnerRegistry;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn run(action: &str) -> Result<(), Box<dyn Error>> {
    match action {
        "install" => {
            // Detect AI tools and install only to those agents
            let agents = detect_agents();
            install(
                if agents.is_empty() {
                    None
                } else {
                    Some(agents.as_slice())
                },
                false,
            )
        }
        "update" => update(),
        "list" => list(),
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        _ => Err(format!("Unknown skill action: {action}").into()),
    }
}

/// Maps runner IDs to skills.sh agent names
pub fn runner_to_skills_agent(runner_id: &str) -> Option<&'static str> {
    match runner_id {
        "claude" => Some("claude-code"),
        "copilot" => Some("github-copilot"),
        "cursor" => Some("cursor"),
        "gemini" => Some("gemini-cli"),
        "codex" => Some("codex"),
        "cline" => Some("cline"),
        "continue" => Some("continue"),
        "windsurf" => Some("windsurf"),
        "aider" => Some("aider"),
        "opencode" => Some("opencode"),
        _ => None,
    }
}

/// Detect installed AI tools and return their skills.sh agent names
fn detect_agents() -> Vec<String> {
    let registry = match RunnerRegistry::load(PathBuf::from(".").as_path()) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    let detections = registry.detect_available(None);
    let agents: Vec<String> = detections
        .iter()
        .filter(|d| d.detected)
        .filter_map(|d| runner_to_skills_agent(&d.runner.id))
        .map(|s| s.to_string())
        .collect();

    if !agents.is_empty() {
        println!(
            "{} Installing to detected tools: {}",
            "•".cyan(),
            agents.join(", ")
        );
    }

    agents
}

/// Install skills, optionally limited to specific agents.
/// If agents is None or empty, installs to all agents (fallback).
/// If skip_confirm is true, passes -y to skip interactive prompts.
pub fn install(agents: Option<&[String]>, skip_confirm: bool) -> Result<(), Box<dyn Error>> {
    let mut args = vec![
        "skills",
        "add",
        "codervisor/lean-spec",
        "--skill",
        "leanspec-sdd",
    ];
    if skip_confirm {
        args.push("-y");
    }

    // Build agent flags if specific agents are provided
    let agent_args: Vec<String>;
    if let Some(agent_list) = agents {
        if !agent_list.is_empty() {
            agent_args = agent_list
                .iter()
                .flat_map(|a| vec!["--agent".to_string(), a.clone()])
                .collect();
            for arg in &agent_args {
                args.push(arg.as_str());
            }
        }
    }

    run_npx(&args)
}

fn update() -> Result<(), Box<dyn Error>> {
    run_npx(&["skills", "update"])
}

fn list() -> Result<(), Box<dyn Error>> {
    run_npx(&["skills", "list"])
}

fn run_npx(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let status = Command::new("npx")
        .args(args)
        .status()
        .map_err(|err| format!("Failed to run npx (is Node.js installed?): {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("npx {} exited with {status}", args.join(" ")).into())
    }
}

fn print_help() {
    println!("{}", "Skill management (skills.sh)".bold());
    println!();
    println!("Usage:");
    println!("  lean-spec skill install   # Install leanspec-sdd via skills.sh");
    println!("  lean-spec skill update    # Update installed skills");
    println!("  lean-spec skill list      # List installed skills");
}
