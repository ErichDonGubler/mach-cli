use std::{env::args_os, path::PathBuf, process::exit};

use anyhow::Context;

fn main() {
    env_logger::builder().parse_env("MACH_SEARCH_LOG").init();

    match find_and_run_mach_script() {
        Ok(()) => (),
        Err(e) => {
            log::error!("{e:?}");
        }
    }
}

fn find_and_run_mach_script() -> anyhow::Result<()> {
    // TODO: detect cycles

    let mach_cmd = search_for_mach_script()
        .context("failed to run search for `mach` script")?
        .context("failed to find `mach` script; see earlier logs for more details")?;

    log::info!("found and using `mach` script at {}", mach_cmd.display());

    let args = args_os().into_iter().skip(1); // First arg. is this binary's name, which we should
                                              // skip.
    match ezcmd::EasyCommand::simple(mach_cmd, args).run() {
        Ok(()) => Ok(()),
        Err(e) => match &e.source {
            // TODO: document default rationale
            ezcmd::RunErrorKind::UnsuccessfulExitCode { code } => exit(code.unwrap_or(1)),
            _other => Err(e.into()),
        },
    }
}

fn search_for_mach_script() -> anyhow::Result<Option<PathBuf>> {
    // TODO: use a Mercurial or Git repository root as the upper bound for searching

    let search_up = {
        log::debug!("searching current and parent directories for a `mach` script…");
        // TODO: ensure we're in a `mozilla-central` checkout first
        let cmd_basename = if cfg!(windows) { "mach.cmd" } else { "mach" };
        lets_find_up::find_up(cmd_basename)
            .context("failed to search in current and parent directories")?
    };
    if search_up.is_some() {
        return Ok(search_up);
    }

    Ok(None)
}
