use std::{path::PathBuf, process::Command};

use anyhow::{bail, Context, Result};

use crate::{find_executable, path_from_env, PHPInfo, PHPProvider};

pub struct Provider<'a> {
    info: &'a PHPInfo,
}

impl<'a> Provider<'a> {
    /// Runs `php-config` with one argument, returning the stdout.
    fn php_config(&self, arg: &str) -> Result<String> {
        let cmd = Command::new(self.find_bin()?)
            .arg(arg)
            .output()
            .context("Failed to run `php-config`")?;
        let stdout = String::from_utf8_lossy(&cmd.stdout);
        if !cmd.status.success() {
            let stderr = String::from_utf8_lossy(&cmd.stderr);
            bail!("Failed to run `php-config`: {} {}", stdout, stderr);
        }
        Ok(stdout.to_string())
    }

    fn find_bin(&self) -> Result<PathBuf> {
        // If path is given via env, it takes priority.
        if let Some(path) = path_from_env("PHP_CONFIG") {
            if !path.try_exists()? {
                // If path was explicitly given and it can't be found, this is a hard error
                bail!("php-config executable not found at {:?}", path);
            }
            return Ok(path);
        }
        find_executable("php-config").with_context(|| {
            "Could not find `php-config` executable. \
            Please ensure `php-config` is in your PATH or the \
            `PHP_CONFIG` environment variable is set."
        })
    }
}

impl<'a> PHPProvider<'a> for Provider<'a> {
    fn new(info: &'a PHPInfo) -> Result<Self> {
        Ok(Self { info })
    }

    fn get_includes(&self) -> Result<Vec<PathBuf>> {
        Ok(self
            .php_config("--includes")?
            .split(' ')
            .map(|s| s.trim_start_matches("-I"))
            .map(PathBuf::from)
            .collect())
    }

    fn get_defines(&self) -> Result<Vec<(&'static str, &'static str)>> {
        let mut defines = vec![];
        if self.info.thread_safety()? {
            defines.push(("ZTS", "1"));
            // defines.push(("ZEND_ENABLE_STATIC_TSRMLS_CACHE", "1"));
        }
        Ok(defines)
    }

    fn print_extra_link_args(&self) -> Result<()> {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=dylib=php");

        Ok(())
    }
}
