use crate::{
    config::{
        parameter::{
            ParamDef, {self},
        },
        CommandConfig, Config, DefaultConfig,
    },
    opt::{Opt, Parameter},
    util::{cstring::to_cstring, value::Referable},
};
use itertools::Itertools;
use nix::unistd;
use std::{
    convert::Infallible,
    ffi::{CString, OsString},
    os::unix::ffi::OsStrExt,
    process::exit,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("command `{0}` is not defined in config.yml")]
    CommandIsNotDefined(String),

    #[error("unknown parameter `{0}`")]
    UnknownParameter(String),

    #[error("ambiguous parameter placement for `{0}`")]
    AmbiguousParameterPlacement(String),

    #[error("parameter `{0}`: argument extraction failed: {1}")]
    ArgumentExtractionFailed(String, parameter::Error),

    #[error("execvp error: {0}")]
    ExecvpeFailed(#[from] nix::Error),
}

///
/// # Panics
///
/// Panics when opt.command is None
pub fn run_command(config: &Config, opt: &Opt) -> Result<Infallible, Error> {
    let command = opt.command.as_ref().expect("must be specified");
    let cmd_config = if let Some(cmd_config) = config.command.get(command) {
        Referable::Borrowed(cmd_config)
    } else if *compose_value(
        &config.default.passthrough_unknown_command,
        &None,
        &opt.passthrough,
    ) {
        Referable::Owned(CommandConfig::empty_for(command))
    } else {
        return Err(Error::CommandIsNotDefined(command.clone()));
    };

    let mut composer = CommandComposer::new(&config.default, &cmd_config);
    for param in &opt.parameters {
        composer.add_parameter(param)?;
    }

    let args = composer.exec_args(&opt.args);
    if opt.debug || opt.verbose || opt.dry_run {
        let args_str = args.iter().map(|arg| format!("{arg:?}")).join(" ");
        if opt.dry_run {
            println!("{args_str}");
            exit(0);
        } else {
            eprintln!("running command: {args_str}");
        }
    }

    unistd::execvp(&args[0], &args).map_err(|e| e.into())
}

fn compose_value<'a, T>(default: &'a T, cmd_val: &'a Option<T>, opt_val: &'a Option<T>) -> &'a T {
    opt_val.as_ref().or(cmd_val.as_ref()).unwrap_or(default)
}

struct CommandComposer<'a> {
    default_config: &'a DefaultConfig,
    cmd_config: &'a CommandConfig,

    head_args: Vec<CString>,
    middle_args: Vec<CString>,
    tail_args: Vec<CString>,
}

impl<'a> CommandComposer<'a> {
    fn new(default_config: &'a DefaultConfig, cmd_config: &'a CommandConfig) -> Self {
        Self {
            default_config,
            cmd_config,

            head_args: Vec::new(),
            middle_args: Vec::new(),
            tail_args: Vec::new(),
        }
    }

    fn add_parameter(&mut self, param: &Parameter) -> Result<(), Error> {
        let placeholder = compose_value(
            &self.default_config.placeholder,
            &self.cmd_config.placeholder,
            &None,
        );
        let (param_def, args) = self.select_param_target(&param.name)?;
        let extracted = param_def
            .extract_as_cstrings(placeholder, &param.value)
            .map_err(|e| Error::ArgumentExtractionFailed(param.name.clone(), e))?;
        args.extend_from_slice(&extracted);
        Ok(())
    }

    fn select_param_target(&mut self, name: &str) -> Result<(&ParamDef, &mut Vec<CString>), Error> {
        match (
            self.cmd_config.headparams.get(name),
            self.cmd_config.middleparams.get(name),
            self.cmd_config.tailparams.get(name),
        ) {
            (Some(def), None, None) => Ok((def, &mut self.head_args)),
            (None, Some(def), None) => Ok((def, &mut self.middle_args)),
            (None, None, Some(def)) => Ok((def, &mut self.tail_args)),
            (None, None, None) => Err(Error::UnknownParameter(name.to_string())),
            _ => Err(Error::AmbiguousParameterPlacement(name.to_string())),
        }
    }

    fn exec_args(&self, extra_args: &[OsString]) -> Vec<CString> {
        let len = self.cmd_config.base.len()
            + self.head_args.len()
            + self.middle_args.len()
            + self.tail_args.len();
        let mut args = Vec::with_capacity(len);
        let base = self.cmd_config.base.as_slice();

        args.push(to_cstring(base[0].as_bytes()));
        args.extend_from_slice(&self.head_args);
        args.extend(base[1..].iter().map(|arg| to_cstring(arg.as_bytes())));
        args.extend_from_slice(&self.middle_args);
        args.extend(extra_args.iter().map(|arg| to_cstring(arg.as_bytes())));
        args.extend_from_slice(&self.tail_args);
        args
    }
}
