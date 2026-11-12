#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}};
use syn::{visit::Visit, Fields, GenericArgument, ImplItem, PathArguments, Type};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "gravita-xtask", about = "Deterministic Gravita engineering audits")]
struct Cli { #[command(subcommand)] command: TopCommand }
#[derive(Subcommand)]
enum TopCommand { Audit { #[command(subcommand)] command: AuditCommand } }
#[derive(Subcommand)]
enum AuditCommand {
    /// Audit rescued operations against typed Rust commands and Bus registration.
    RescuedCommands {
        #[arg(long, default_value = "docs/contracts/rescued-commands.json")]
        contract: PathBuf,
        #[arg(long, default_value = "crates")]
        source_root: PathBuf,
        #[arg(long)]
        json: Option<PathBuf>,
    },
}
#[derive(Deserialize)]
struct Contract { contract_id: String, commands: Vec<ContractCommand> }
#[derive(Deserialize)]
struct ContractCommand { id: String, name: String, parameters: Vec<ContractParameter> }
#[derive(Deserialize)]
struct ContractParameter { name: String, #[serde(rename = "type")] parameter_type: serde_json::Value }
#[derive(Clone)]
struct Payload { type_name: String, fields: BTreeMap<String, String>, file: String }
#[derive(Default)]
struct SourceIndex {
    payloads: BTreeMap<String, Payload>,
    command_ids: BTreeMap<String, Vec<String>>,
    command_impls: BTreeSet<String>,
    registrations: BTreeSet<String>,
}
struct Collector<'a> { file: &'a str, index: &'a mut SourceIndex }

fn type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path.path.segments.last().map(|s| s.ident.to_string()),
        Type::Reference(reference) => type_name(&reference.elem),
        Type::Group(group) => type_name(&group.elem),
        Type::Paren(paren) => type_name(&paren.elem),
        _ => None,
    }
}
fn type_tokens(ty: &Type) -> String { format!("{ty:?}") }
fn rust_type_family(raw: &str) -> &'static str {
    let s = raw.to_ascii_lowercase();
    if s.contains("f32") || s.contains("f64") { "Float" }
    else if ["i8","i16","i32","i64","u8","u16","u32","u64","usize","isize"].iter().any(|n| s.contains(n)) { "Int" }
    else if s.contains("bool") { "Bool" }
    else if s.contains("string") || s.contains("str") { "String" }
    else { "Other" }
}
fn contract_type_family(value: &serde_json::Value) -> &'static str {
    value.get("type").and_then(serde_json::Value::as_str).unwrap_or("Other")
}
impl<'ast> Visit<'ast> for Collector<'_> {
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        let fields = match &item.fields {
            Fields::Named(named) => named.named.iter().filter_map(|field| {
                Some((field.ident.as_ref()?.to_string(), type_tokens(&field.ty)))
            }).collect(),
            _ => BTreeMap::new(),
        };
        self.index.payloads.insert(item.ident.to_string(), Payload {
            type_name: item.ident.to_string(), fields, file: self.file.to_owned(),
        });
        syn::visit::visit_item_struct(self, item);
    }
    fn visit_expr_call(&mut self, expression: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = expression.func.as_ref() {
            if let Some(segment) = function.path.segments.last() {
                if segment.ident == "register" {
                    if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                        for argument in &arguments.args {
                            if let GenericArgument::Type(ty) = argument {
                                if let Some(name) = type_name(ty) {
                                    self.index.registrations.insert(name);
                                }
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_call(self, expression);
    }
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if let Some((_, path, _)) = &item.trait_ {
            if path.segments.last().is_some_and(|s| s.ident == "Command") {
                if let Some(name) = type_name(&item.self_ty) {
                    self.index.command_impls.insert(name.clone());
                    for member in &item.items {
                        if let ImplItem::Const(value) = member {
                            if value.ident == "ID" {
                                if let syn::Expr::Lit(expr) = &value.expr {
                                    if let syn::Lit::Str(id) = &expr.lit {
                                        self.index.command_ids.entry(id.value()).or_default().push(name.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_item_impl(self, item);
    }
}
#[derive(Serialize)]
struct Issue { command_id: String, code: String, detail: String }
#[derive(Serialize)]
struct Report {
    contract_id: String, contract_commands: usize, declarations_found: usize,
    implemented_commands: usize, registered_commands: usize, passed: bool, issues: Vec<Issue>,
}
fn collect_sources(root: &Path) -> Result<SourceIndex> {
    if !root.exists() { return Ok(SourceIndex::default()); }
    let mut index = SourceIndex::default();
    for entry in WalkDir::new(root) {
        let entry = entry.with_context(|| format!("walking {}", root.display()))?;
        let path = entry.path();
        if !entry.file_type().is_file() || path.extension() != Some(std::ffi::OsStr::new("rs")) { continue; }
        let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let syntax = syn::parse_file(&source).with_context(|| format!("parsing Rust source {}", path.display()))?;
        let display = path.display().to_string();
        let mut collector = Collector { file: &display, index: &mut index };
        collector.visit_file(&syntax);
    }
    Ok(index)
}
fn audit(contract_path: &Path, source_root: &Path, json_path: Option<&Path>) -> Result<()> {
    let raw = fs::read_to_string(contract_path).with_context(|| format!("reading {}", contract_path.display()))?;
    let contract: Contract = serde_json::from_str(&raw).with_context(|| format!("parsing {}", contract_path.display()))?;
    let index = collect_sources(source_root)?;
    let mut issues = Vec::new();
    let mut declarations_found = 0;
    let mut implemented_commands = 0;
    let mut registered_commands = 0;
    for command in &contract.commands {
        let Some(type_names) = index.command_ids.get(&command.id) else {
            issues.push(Issue { command_id: command.id.clone(), code: "missing_command_impl".into(),
                detail: format!("No impl Command declares ID {} (expected an associated const ID).", command.id) });
            continue;
        };
        if type_names.len() != 1 {
            issues.push(Issue { command_id: command.id.clone(), code: "duplicate_command_id".into(),
                detail: format!("Command ID {} is declared by multiple types: {}.", command.id, type_names.join(", ")) });
            continue;
        }
        let type_name = &type_names[0];
        let Some(payload) = index.payloads.get(type_name) else {
            issues.push(Issue { command_id: command.id.clone(), code: "missing_payload".into(),
                detail: format!("Command type {} has no Rust struct payload.", type_name) });
            continue;
        };
        declarations_found += 1;
        if index.command_impls.contains(type_name) { implemented_commands += 1; }
        if index.registrations.contains(type_name) { registered_commands += 1; }
        else {
            issues.push(Issue { command_id: command.id.clone(), code: "missing_bus_registration".into(),
                detail: format!("{} is not registered with CommandBus::register::<{}>() in the dispatcher.", type_name, type_name) });
        }
        let expected: BTreeMap<_, _> = command.parameters.iter()
            .map(|p| (p.name.as_str(), contract_type_family(&p.parameter_type))).collect();
        for (name, expected_type) in &expected {
            match payload.fields.get(*name) {
                None => issues.push(Issue { command_id: command.id.clone(), code: "missing_parameter".into(),
                    detail: format!("Payload {} in {} has no field {}.", type_name, payload.file, name) }),
                Some(actual) => {
                    let actual_type = rust_type_family(actual);
                    if actual_type != "Other" && actual_type != *expected_type
                        && !(*expected_type == "Choice" && actual_type == "String") {
                        issues.push(Issue { command_id: command.id.clone(), code: "parameter_type_mismatch".into(),
                            detail: format!("Parameter {} expects {}, but {} is {}.", name, expected_type, type_name, actual) });
                    }
                }
            }
        }
        for name in payload.fields.keys() {
            if !expected.contains_key(name.as_str()) {
                issues.push(Issue { command_id: command.id.clone(), code: "undocumented_parameter".into(),
                    detail: format!("Payload {} contains {}, absent from the contract.", type_name, name) });
            }
        }
    }
    let report = Report { contract_id: contract.contract_id, contract_commands: contract.commands.len(),
        declarations_found, implemented_commands, registered_commands,
        passed: issues.is_empty() && declarations_found == contract.commands.len()
            && implemented_commands == contract.commands.len() && registered_commands == contract.commands.len(),
        issues };
    let rendered = serde_json::to_string_pretty(&report)?;
    if let Some(path) = json_path {
        if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
        let temp = path.with_extension("json.tmp");
        fs::write(&temp, format!("{rendered}\n"))?;
        fs::rename(temp, path)?;
    }
    println!("{rendered}");
    if !report.passed { anyhow::bail!("rescued-command audit failed with {} issue(s)", report.issues.len()); }
    Ok(())
}
fn main() -> Result<()> {
    match Cli::parse().command {
        TopCommand::Audit { command: AuditCommand::RescuedCommands { contract, source_root, json } } =>
            audit(&contract, &source_root, json.as_deref()),
    }
}
