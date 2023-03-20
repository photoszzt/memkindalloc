use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[allow(unused)]
const ALLOWLIST_FUNCTION: &'static [&'static str] = &["memkind_.*"];

#[allow(unused)]
const ALLOWLIST_TYPES: &'static [&'static str] = &["memkind_.*"];

macro_rules! info {
    ($($args:tt)*) => { println!($($args)*) }
}

fn run_and_log(cmd: &mut Command, log_file: &Path) {
    execute(cmd, || {
        run(Command::new("tail").arg("-n").arg("100").arg(log_file));
    })
}

fn run(cmd: &mut Command) {
    execute(cmd, || ());
}

fn execute(cmd: &mut Command, on_fail: impl FnOnce()) {
    println!("running: {cmd:?}");
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute command: {}", e),
    };
    if !status.success() {
        on_fail();
        panic!(
            "command did not execute successfully: {:?}\n\
             expected success, got: {}",
            cmd, status
        );
    }
}

fn main() {
    let num_jobs = env::var("NUM_JOBS").expect("NUM_JOBS was not set");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
    let src_dir = env::current_dir().expect("failed to get current directory");
    let build_dir = out_dir.join("build");
    let install_dir = src_dir.join("memkind_install");

    fs::remove_dir_all(out_dir.clone()).unwrap();
    fs::create_dir_all(&out_dir).unwrap();
    fs::create_dir_all(&build_dir).unwrap();
    fs::create_dir_all(&install_dir).unwrap();
    let memkind_dir = src_dir.join("memkind");

    info!("OUT_DIR={:?}", out_dir);
    info!("BUILD_DIR={:?}", build_dir);
    info!("SRC_DIR={:?}", src_dir);

    let compiler = cc::Build::new().extra_warnings(false).get_compiler();
    let cflags = compiler
        .args()
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    info!("CC={:?}", compiler.path());
    info!("CFLAGS={:?}", cflags);

    assert!(out_dir.exists(), "OUT_DIR does not exist");
    info!("MEMKIND_REPO_DIR={:?}", memkind_dir);

    // Configuration files
    let config_files = ["VERSION"];

    // Copy the configuration files to jemalloc's source directory
    for f in &config_files {
        fs::copy(Path::new("configure").join(f), memkind_dir.join(f))
            .expect("failed to copy config file to memkind dir");
    }

    // Run autogen:
    let autogen = memkind_dir.join("autogen.sh");
    let mut autogen_cmd = Command::new("sh");
    autogen_cmd
        .arg(
            autogen
                .to_str()
                .unwrap()
                .replace("C:\\", "/c/")
                .replace('\\', "/"),
        )
        .current_dir(&memkind_dir);

    run_and_log(&mut autogen_cmd, &build_dir.join("autogen.log"));

    // Run configure:
    let configure = memkind_dir.join("configure");
    let mut cmd = Command::new("sh");
    cmd.arg(
        configure
            .to_str()
            .unwrap()
            .replace("C:\\", "/c/")
            .replace('\\', "/"),
    )
    .current_dir(&memkind_dir)
    .env("CC", compiler.path())
    .env("CFLAGS", cflags.clone())
    .env("LDFLAGS", cflags.clone())
    .env("CPPFLAGS", cflags)
    .arg(format!("--prefix={}", install_dir.display()));

    run_and_log(&mut cmd, &build_dir.join("config.log"));

    // Make:
    run(Command::new("make")
        .current_dir(&memkind_dir)
        .arg("-j")
        .arg(num_jobs.clone()));

    // Make install:
    run(Command::new("make")
        .current_dir(&memkind_dir)
        .arg("install"));

    #[cfg(feature = "generate")]
    {
        build_memkind_bindings(&install_dir);
    }

    link_info();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        install_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        memkind_dir.display()
    );
    println!(
        "cargo:rerun-if-changed=native={}/src",
        memkind_dir.display()
    );
    println!(
        "cargo:rerun-if-changed=native={}/include",
        memkind_dir.display()
    );
    println!("cargo:root={}", out_dir.display());
}

fn link_info() {
    println!("cargo:rustc-link-lib=memkind");
}

// define the function for when we will generate bindings
#[cfg(feature = "generate")]
fn build_memkind_bindings(out_dir: &PathBuf) {
    let include_dir = out_dir.join("include");
    let header_file = include_dir.join("memkind.h");
    let mut bindings = bindgen::Builder::default()
        .header(header_file.display().to_string())
        .clang_arg("-I")
        .clang_arg(include_dir.display().to_string());
    for func in ALLOWLIST_FUNCTION {
        bindings = bindings.allowlist_function(func);
    }

    for ty in ALLOWLIST_TYPES {
        bindings = bindings.allowlist_type(ty);
    }
    bindings = bindings
        .size_t_is_usize(true)
        .derive_debug(true)
        .impl_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .impl_partialeq(true)
        .derive_eq(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_hash(true)
        .prepend_enum_name(false)
        .rustfmt_bindings(true);
    let builder = bindings
        .generate()
        .expect("Should generate memkind API bindings OK");
    builder
        .write_to_file("src/memkind_api.rs")
        .expect("Couldn't write bcc bindings!");

    let have_working_rustfmt = std::process::Command::new("rustup")
        .args(&["run", "rustfmt", "--version"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok()
        .map_or(false, |status| status.success());

    if !have_working_rustfmt {
        println!(
            "
        The latest `rustfmt` is required to format the generated bindings. Install
            `rustfmt` with:
            $ rustup component add rustfmt
            $ rustup update
            "
        );
    }
}
