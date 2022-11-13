// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gobject_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gobject-2.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {cmd:?} returned {}", out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {abi_cmd:?} failed, {output:?}").into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GBindingFlags",
        Layout {
            size: size_of::<GBindingFlags>(),
            alignment: align_of::<GBindingFlags>(),
        },
    ),
    (
        "GClosureNotifyData",
        Layout {
            size: size_of::<GClosureNotifyData>(),
            alignment: align_of::<GClosureNotifyData>(),
        },
    ),
    (
        "GConnectFlags",
        Layout {
            size: size_of::<GConnectFlags>(),
            alignment: align_of::<GConnectFlags>(),
        },
    ),
    (
        "GEnumClass",
        Layout {
            size: size_of::<GEnumClass>(),
            alignment: align_of::<GEnumClass>(),
        },
    ),
    (
        "GEnumValue",
        Layout {
            size: size_of::<GEnumValue>(),
            alignment: align_of::<GEnumValue>(),
        },
    ),
    (
        "GFlagsClass",
        Layout {
            size: size_of::<GFlagsClass>(),
            alignment: align_of::<GFlagsClass>(),
        },
    ),
    (
        "GFlagsValue",
        Layout {
            size: size_of::<GFlagsValue>(),
            alignment: align_of::<GFlagsValue>(),
        },
    ),
    (
        "GInitiallyUnowned",
        Layout {
            size: size_of::<GInitiallyUnowned>(),
            alignment: align_of::<GInitiallyUnowned>(),
        },
    ),
    (
        "GInitiallyUnownedClass",
        Layout {
            size: size_of::<GInitiallyUnownedClass>(),
            alignment: align_of::<GInitiallyUnownedClass>(),
        },
    ),
    (
        "GInterfaceInfo",
        Layout {
            size: size_of::<GInterfaceInfo>(),
            alignment: align_of::<GInterfaceInfo>(),
        },
    ),
    (
        "GObject",
        Layout {
            size: size_of::<GObject>(),
            alignment: align_of::<GObject>(),
        },
    ),
    (
        "GObjectClass",
        Layout {
            size: size_of::<GObjectClass>(),
            alignment: align_of::<GObjectClass>(),
        },
    ),
    (
        "GObjectConstructParam",
        Layout {
            size: size_of::<GObjectConstructParam>(),
            alignment: align_of::<GObjectConstructParam>(),
        },
    ),
    (
        "GParamFlags",
        Layout {
            size: size_of::<GParamFlags>(),
            alignment: align_of::<GParamFlags>(),
        },
    ),
    (
        "GParamSpec",
        Layout {
            size: size_of::<GParamSpec>(),
            alignment: align_of::<GParamSpec>(),
        },
    ),
    (
        "GParamSpecBoolean",
        Layout {
            size: size_of::<GParamSpecBoolean>(),
            alignment: align_of::<GParamSpecBoolean>(),
        },
    ),
    (
        "GParamSpecBoxed",
        Layout {
            size: size_of::<GParamSpecBoxed>(),
            alignment: align_of::<GParamSpecBoxed>(),
        },
    ),
    (
        "GParamSpecChar",
        Layout {
            size: size_of::<GParamSpecChar>(),
            alignment: align_of::<GParamSpecChar>(),
        },
    ),
    (
        "GParamSpecClass",
        Layout {
            size: size_of::<GParamSpecClass>(),
            alignment: align_of::<GParamSpecClass>(),
        },
    ),
    (
        "GParamSpecDouble",
        Layout {
            size: size_of::<GParamSpecDouble>(),
            alignment: align_of::<GParamSpecDouble>(),
        },
    ),
    (
        "GParamSpecEnum",
        Layout {
            size: size_of::<GParamSpecEnum>(),
            alignment: align_of::<GParamSpecEnum>(),
        },
    ),
    (
        "GParamSpecFlags",
        Layout {
            size: size_of::<GParamSpecFlags>(),
            alignment: align_of::<GParamSpecFlags>(),
        },
    ),
    (
        "GParamSpecFloat",
        Layout {
            size: size_of::<GParamSpecFloat>(),
            alignment: align_of::<GParamSpecFloat>(),
        },
    ),
    (
        "GParamSpecGType",
        Layout {
            size: size_of::<GParamSpecGType>(),
            alignment: align_of::<GParamSpecGType>(),
        },
    ),
    (
        "GParamSpecInt",
        Layout {
            size: size_of::<GParamSpecInt>(),
            alignment: align_of::<GParamSpecInt>(),
        },
    ),
    (
        "GParamSpecInt64",
        Layout {
            size: size_of::<GParamSpecInt64>(),
            alignment: align_of::<GParamSpecInt64>(),
        },
    ),
    (
        "GParamSpecLong",
        Layout {
            size: size_of::<GParamSpecLong>(),
            alignment: align_of::<GParamSpecLong>(),
        },
    ),
    (
        "GParamSpecObject",
        Layout {
            size: size_of::<GParamSpecObject>(),
            alignment: align_of::<GParamSpecObject>(),
        },
    ),
    (
        "GParamSpecOverride",
        Layout {
            size: size_of::<GParamSpecOverride>(),
            alignment: align_of::<GParamSpecOverride>(),
        },
    ),
    (
        "GParamSpecParam",
        Layout {
            size: size_of::<GParamSpecParam>(),
            alignment: align_of::<GParamSpecParam>(),
        },
    ),
    (
        "GParamSpecPointer",
        Layout {
            size: size_of::<GParamSpecPointer>(),
            alignment: align_of::<GParamSpecPointer>(),
        },
    ),
    (
        "GParamSpecTypeInfo",
        Layout {
            size: size_of::<GParamSpecTypeInfo>(),
            alignment: align_of::<GParamSpecTypeInfo>(),
        },
    ),
    (
        "GParamSpecUChar",
        Layout {
            size: size_of::<GParamSpecUChar>(),
            alignment: align_of::<GParamSpecUChar>(),
        },
    ),
    (
        "GParamSpecUInt",
        Layout {
            size: size_of::<GParamSpecUInt>(),
            alignment: align_of::<GParamSpecUInt>(),
        },
    ),
    (
        "GParamSpecUInt64",
        Layout {
            size: size_of::<GParamSpecUInt64>(),
            alignment: align_of::<GParamSpecUInt64>(),
        },
    ),
    (
        "GParamSpecULong",
        Layout {
            size: size_of::<GParamSpecULong>(),
            alignment: align_of::<GParamSpecULong>(),
        },
    ),
    (
        "GParamSpecUnichar",
        Layout {
            size: size_of::<GParamSpecUnichar>(),
            alignment: align_of::<GParamSpecUnichar>(),
        },
    ),
    (
        "GParamSpecValueArray",
        Layout {
            size: size_of::<GParamSpecValueArray>(),
            alignment: align_of::<GParamSpecValueArray>(),
        },
    ),
    (
        "GParamSpecVariant",
        Layout {
            size: size_of::<GParamSpecVariant>(),
            alignment: align_of::<GParamSpecVariant>(),
        },
    ),
    (
        "GParameter",
        Layout {
            size: size_of::<GParameter>(),
            alignment: align_of::<GParameter>(),
        },
    ),
    (
        "GSignalCMarshaller",
        Layout {
            size: size_of::<GSignalCMarshaller>(),
            alignment: align_of::<GSignalCMarshaller>(),
        },
    ),
    (
        "GSignalFlags",
        Layout {
            size: size_of::<GSignalFlags>(),
            alignment: align_of::<GSignalFlags>(),
        },
    ),
    (
        "GSignalInvocationHint",
        Layout {
            size: size_of::<GSignalInvocationHint>(),
            alignment: align_of::<GSignalInvocationHint>(),
        },
    ),
    (
        "GSignalMatchType",
        Layout {
            size: size_of::<GSignalMatchType>(),
            alignment: align_of::<GSignalMatchType>(),
        },
    ),
    (
        "GSignalQuery",
        Layout {
            size: size_of::<GSignalQuery>(),
            alignment: align_of::<GSignalQuery>(),
        },
    ),
    (
        "GTypeClass",
        Layout {
            size: size_of::<GTypeClass>(),
            alignment: align_of::<GTypeClass>(),
        },
    ),
    (
        "GTypeDebugFlags",
        Layout {
            size: size_of::<GTypeDebugFlags>(),
            alignment: align_of::<GTypeDebugFlags>(),
        },
    ),
    (
        "GTypeFlags",
        Layout {
            size: size_of::<GTypeFlags>(),
            alignment: align_of::<GTypeFlags>(),
        },
    ),
    (
        "GTypeFundamentalFlags",
        Layout {
            size: size_of::<GTypeFundamentalFlags>(),
            alignment: align_of::<GTypeFundamentalFlags>(),
        },
    ),
    (
        "GTypeFundamentalInfo",
        Layout {
            size: size_of::<GTypeFundamentalInfo>(),
            alignment: align_of::<GTypeFundamentalInfo>(),
        },
    ),
    (
        "GTypeInfo",
        Layout {
            size: size_of::<GTypeInfo>(),
            alignment: align_of::<GTypeInfo>(),
        },
    ),
    (
        "GTypeInstance",
        Layout {
            size: size_of::<GTypeInstance>(),
            alignment: align_of::<GTypeInstance>(),
        },
    ),
    (
        "GTypeInterface",
        Layout {
            size: size_of::<GTypeInterface>(),
            alignment: align_of::<GTypeInterface>(),
        },
    ),
    (
        "GTypeModule",
        Layout {
            size: size_of::<GTypeModule>(),
            alignment: align_of::<GTypeModule>(),
        },
    ),
    (
        "GTypeModuleClass",
        Layout {
            size: size_of::<GTypeModuleClass>(),
            alignment: align_of::<GTypeModuleClass>(),
        },
    ),
    (
        "GTypePluginClass",
        Layout {
            size: size_of::<GTypePluginClass>(),
            alignment: align_of::<GTypePluginClass>(),
        },
    ),
    (
        "GTypeQuery",
        Layout {
            size: size_of::<GTypeQuery>(),
            alignment: align_of::<GTypeQuery>(),
        },
    ),
    (
        "GTypeValueTable",
        Layout {
            size: size_of::<GTypeValueTable>(),
            alignment: align_of::<GTypeValueTable>(),
        },
    ),
    (
        "GValue",
        Layout {
            size: size_of::<GValue>(),
            alignment: align_of::<GValue>(),
        },
    ),
    (
        "GValueArray",
        Layout {
            size: size_of::<GValueArray>(),
            alignment: align_of::<GValueArray>(),
        },
    ),
    (
        "GWeakRef",
        Layout {
            size: size_of::<GWeakRef>(),
            alignment: align_of::<GWeakRef>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) G_BINDING_BIDIRECTIONAL", "1"),
    ("(guint) G_BINDING_DEFAULT", "0"),
    ("(guint) G_BINDING_INVERT_BOOLEAN", "4"),
    ("(guint) G_BINDING_SYNC_CREATE", "2"),
    ("(guint) G_CONNECT_AFTER", "1"),
    ("(guint) G_CONNECT_DEFAULT", "0"),
    ("(guint) G_CONNECT_SWAPPED", "2"),
    ("(guint) G_PARAM_CONSTRUCT", "4"),
    ("(guint) G_PARAM_CONSTRUCT_ONLY", "8"),
    ("(guint) G_PARAM_DEPRECATED", "2147483648"),
    ("(guint) G_PARAM_EXPLICIT_NOTIFY", "1073741824"),
    ("(guint) G_PARAM_LAX_VALIDATION", "16"),
    ("G_PARAM_MASK", "255"),
    ("(guint) G_PARAM_PRIVATE", "32"),
    ("(guint) G_PARAM_READABLE", "1"),
    ("(guint) G_PARAM_READWRITE", "3"),
    ("(guint) G_PARAM_STATIC_BLURB", "128"),
    ("(guint) G_PARAM_STATIC_NAME", "32"),
    ("(guint) G_PARAM_STATIC_NICK", "64"),
    ("G_PARAM_STATIC_STRINGS", "224"),
    ("G_PARAM_USER_SHIFT", "8"),
    ("(guint) G_PARAM_WRITABLE", "2"),
    ("(guint) G_SIGNAL_ACCUMULATOR_FIRST_RUN", "131072"),
    ("(guint) G_SIGNAL_ACTION", "32"),
    ("(guint) G_SIGNAL_DEPRECATED", "256"),
    ("(guint) G_SIGNAL_DETAILED", "16"),
    ("G_SIGNAL_FLAGS_MASK", "511"),
    ("(guint) G_SIGNAL_MATCH_CLOSURE", "4"),
    ("(guint) G_SIGNAL_MATCH_DATA", "16"),
    ("(guint) G_SIGNAL_MATCH_DETAIL", "2"),
    ("(guint) G_SIGNAL_MATCH_FUNC", "8"),
    ("(guint) G_SIGNAL_MATCH_ID", "1"),
    ("G_SIGNAL_MATCH_MASK", "63"),
    ("(guint) G_SIGNAL_MATCH_UNBLOCKED", "32"),
    ("(guint) G_SIGNAL_MUST_COLLECT", "128"),
    ("(guint) G_SIGNAL_NO_HOOKS", "64"),
    ("(guint) G_SIGNAL_NO_RECURSE", "8"),
    ("(guint) G_SIGNAL_RUN_CLEANUP", "4"),
    ("(guint) G_SIGNAL_RUN_FIRST", "1"),
    ("(guint) G_SIGNAL_RUN_LAST", "2"),
    ("(guint) G_TYPE_DEBUG_INSTANCE_COUNT", "4"),
    ("(guint) G_TYPE_DEBUG_MASK", "7"),
    ("(guint) G_TYPE_DEBUG_NONE", "0"),
    ("(guint) G_TYPE_DEBUG_OBJECTS", "1"),
    ("(guint) G_TYPE_DEBUG_SIGNALS", "2"),
    ("(guint) G_TYPE_FLAG_ABSTRACT", "16"),
    ("(guint) G_TYPE_FLAG_CLASSED", "1"),
    ("(guint) G_TYPE_FLAG_DEEP_DERIVABLE", "8"),
    ("(guint) G_TYPE_FLAG_DERIVABLE", "4"),
    ("(guint) G_TYPE_FLAG_FINAL", "64"),
    ("(guint) G_TYPE_FLAG_INSTANTIATABLE", "2"),
    ("(guint) G_TYPE_FLAG_NONE", "0"),
    ("G_TYPE_FLAG_RESERVED_ID_BIT", "1"),
    ("(guint) G_TYPE_FLAG_VALUE_ABSTRACT", "32"),
    ("G_TYPE_FUNDAMENTAL_MAX", "255"),
    ("G_TYPE_FUNDAMENTAL_SHIFT", "2"),
    ("G_TYPE_RESERVED_BSE_FIRST", "32"),
    ("G_TYPE_RESERVED_BSE_LAST", "48"),
    ("G_TYPE_RESERVED_GLIB_FIRST", "22"),
    ("G_TYPE_RESERVED_GLIB_LAST", "31"),
    ("G_TYPE_RESERVED_USER_FIRST", "49"),
    ("G_VALUE_INTERNED_STRING", "268435456"),
    ("G_VALUE_NOCOPY_CONTENTS", "134217728"),
];
