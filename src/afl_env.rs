// -----------------------------------------
// AFLPlusPlus flags
// Based on: https://aflplus.plus/docs/env_variables/
// -----------------------------------------
use std::{collections::HashSet, str::FromStr};

/// Enum representing the different AFL environment flags
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AFLFlag {
    /// `AFL_AUTORESUME` will resume a fuzz run (same as providing -i -) for an existing out folder, even if a different -i was provided.
    /// Without this setting, afl-fuzz will refuse execution for a long-fuzzed out dir.
    AutoResume,
    /// `AFL_FINAL_SYNC` will cause the fuzzer to perform a final import of test cases when terminating.
    /// This is beneficial for -M main fuzzers to ensure it has all unique test cases and hence you only need to afl-cmin this single queue.
    FinalSync,
    /// Setting `AFL_DISABLE_TRIM` tells afl-fuzz not to trim test cases.
    DisableTrim,
    /// Setting `AFL_KEEP_TIMEOUTS` will keep longer running inputs if they reach new coverage.
    KeepTimeouts,
    /// Setting `AFL_EXPAND_HAVOC_NOW` will start in the extended havoc mode that includes costly mutations.
    /// afl-fuzz automatically enables this mode when deemed useful otherwise.
    ExpandHavocNow,
    /// `AFL_IGNORE_SEED_PROBLEMS` will skip over crashes and timeouts in the seeds instead of exiting.
    IgnoreSeedProblems,
    /// When setting `AFL_IMPORT_FIRST`, test cases from other fuzzers in the campaign are loaded first.
    /// Note: This can slow down the start of the first fuzz by quite a lot if you have many fuzzers and/or many seeds.
    ImportFirst,
}

impl std::fmt::Display for AFLFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::AutoResume => "AFL_AUTORESUME",
            Self::FinalSync => "AFL_FINAL_SYNC",
            Self::DisableTrim => "AFL_DISABLE_TRIM",
            Self::KeepTimeouts => "AFL_KEEP_TIMEOUTS",
            Self::ExpandHavocNow => "AFL_EXPAND_HAVOC_NOW",
            Self::IgnoreSeedProblems => "AFL_IGNORE_SEED_PROBLEMS",
            Self::ImportFirst => "AFL_IMPORT_FIRST",
        };
        write!(f, "{}", x)
    }
}

impl FromStr for AFLFlag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AFL_AUTORESUME" => Ok(Self::AutoResume),
            "AFL_FINAL_SYNC" => Ok(Self::FinalSync),
            "AFL_DISABLE_TRIM" => Ok(Self::DisableTrim),
            "AFL_KEEP_TIMEOUTS" => Ok(Self::KeepTimeouts),
            "AFL_EXPAND_HAVOC_NOW" => Ok(Self::ExpandHavocNow),
            "AFL_IGNORE_SEED_PROBLEMS" => Ok(Self::IgnoreSeedProblems),
            "AFL_IMPORT_FIRST" => Ok(Self::ImportFirst),
            _ => Err(format!("Unknown AFL flag: {s}")),
        }
    }
}

/// Struct representing the environment variables for `AFLPlusPlus`
#[derive(Debug, Clone, Default)]
pub struct AFLEnv {
    /// Set of enabled AFL flags
    pub flags: HashSet<AFLFlag>,
    /// `AFL_TESTCACHE_SIZE` sets caching of test cases in MB (default: 50).
    /// If enough RAM is available, it is recommended to target values between 50-500MB.
    pub testcache_size: u32,
}

impl AFLEnv {
    /// Creates a new `AFLEnv` instance with default values
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
            testcache_size: 50,
        }
    }

    /// Enables the specified AFL flag
    pub fn enable_flag(&mut self, flag: AFLFlag) {
        self.flags.insert(flag);
    }

    /// Generates an `AFLPlusPlus` environment variable string for the current settings
    pub fn generate_afl_env_cmd(&self, ramdisk: Option<String>) -> Vec<String> {
        let mut command = Vec::new();

        if let Some(ramdisk) = ramdisk {
            command.push(format!("AFL_TMPDIR={ramdisk} "));
        }

        for flag in &self.flags {
            command.push(format!("{}=1", flag.to_string()));
        }

        command.push(format!("AFL_TESTCACHE_SIZE={} ", self.testcache_size));

        command
    }
}
