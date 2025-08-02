use std::process::Command;

fn main() {
    #[cfg(all(feature = "static", feature = "shared"))]
    compile_error!("You have enabled both 'static' and 'shared' features, which conflicts. You must choose one linking method: 'static' for static linking, or 'shared' for dynamic linking.");

    #[cfg(not(any(feature = "static", feature = "shared")))]
    compile_error!("You must enable exactly one of the features: 'static' or 'shared'.");

    #[cfg(all(feature = "static", feature = "override", target_os = "windows"))]
    compile_error!("It is only possible to override malloc on Windows when building as a DLL.");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mimalloc/");

    let mut cfg = cmake::Config::new("mimalloc");

    macro_rules! feat_opt {
        ($feat:literal, $opt:literal) => {
            #[cfg(feature = $feat)]
            cfg.define($opt, "ON");
        };

        ($feat:literal, $body:block) => {
            #[cfg(feature = $feat)]
            $body
        };
    }

    let link_type: &str;

    feat_opt!("static", {
        cfg.define("MI_BUILD_STATIC", "ON");
        cfg.define("MI_BUILD_SHARED", "OFF");
        link_type = "static";
    });

    feat_opt!("shared", {
        cfg.define("MI_BUILD_STATIC", "OFF");
        cfg.define("MI_BUILD_SHARED", "ON");
        link_type = "dylib";
    });

    feat_opt!("secure", "MI_SECURE");
    feat_opt!("debug_full", "MI_DEBUG_FULL");
    feat_opt!("padding", "MI_PADDING");
    feat_opt!("override", "MI_OVERRIDE");
    feat_opt!("xmalloc", "MI_XMALLOC");
    feat_opt!("show_errors", "MI_SHOW_ERRORS");
    feat_opt!("guarded", "MI_GUARDED");
    feat_opt!("use_cxx", "MI_USE_CXX");
    feat_opt!("opt_arch", "MI_OPT_ARCH");
    feat_opt!("opt_simd", {
        cfg.define("MI_OPT_SIMD", "ON");
        cfg.cflag("-flax-vector-conversions");
    });
    feat_opt!("see_asm", "MI_SEE_ASM");

    #[cfg(target_os = "macos")]
    {
        feat_opt!("osx_interpose", {
            cfg.define("MI_OSX_INTERPOSE", "ON");
            #[cfg(all(feature = "shared", feature = "use_cxx"))]
            println!("cargo:warning=if dynamically overriding malloc/free, it is more reliable to build mimalloc as C code (don't enable feature use-cxx)")
        });
        feat_opt!("osx_zone", "MI_OSX_ZONE");
    }

    #[cfg(target_os = "windows")]
    {
        feat_opt!("win_redirect", "MI_WIN_REDIRECT");
        feat_opt!("win_use_fixed_tls", "MI_WIN_USE_FIXED_TLS");
    }

    #[cfg(target_family = "unix")]
    feat_opt!("local_dynamic_tls", "MI_LOCAL_DYNAMIC_TLS");

    #[cfg(target_env = "musl")]
    cfg.define("MI_LIBC_MUSL", "ON");

    feat_opt!("debug_asan", "MI_DEBUG_ASAN");
    feat_opt!("debug_ubsan", "MI_DEBUG_UBSAN");
    feat_opt!("track_valgrind", "MI_TRACK_VALGRIND");
    feat_opt!("track_asan", "MI_TRACK_ASAN");
    feat_opt!("track_etw", "MI_TRACK_ETW");

    cfg.define("MI_BUILD_OBJECT", "OFF");
    feat_opt!("build_object", "MI_BUILD_OBJECT");

    cfg.define("MI_BUILD_TESTS", "OFF");
    feat_opt!("build_tests", "MI_BUILD_TESTS");

    feat_opt!("skip_collect_on_exit", "MI_SKIP_COLLECT_ON_EXIT");
    feat_opt!("no_padding", "MI_NO_PADDING");

    #[cfg(any(target_os = "linux", target_os = "android"))]
    feat_opt!("no_thp", "MI_NO_THP");

    // When using a GNU-like compiler, the `__thread` keyword for thread-local storage
    // (as used in mimalloc) implies a dependency on the `__emutls_get_address` symbol.
    // We must explicitly link against libgcc_eh to provide this symbol.
    if cc::Build::new().get_compiler().is_like_gnu() {
        println!("cargo:rustc-link-lib=gcc_eh");
    }

    // Workaround for Apple Silicon (aarch64) macOS.
    // The default MI_ARENA_SLICE_SIZE in mimalloc fails to satisfy the
    // `_mi_os_page_size() <= (MI_ARENA_SLICE_SIZE / 8)` assertion on this platform.
    // This script dynamically retrieves the OS page size and overrides the macro
    // to ensure the assertion passes.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let output = Command::new("getconf").arg("PAGESIZE").output().unwrap();
        if output.status.success() {
            let pagesize: usize = std::str::from_utf8(&output.stdout).unwrap().trim().parse().unwrap();
            let arena_slice_size = pagesize * 8;
            let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
            let header_path = out_dir.join("override_mi_arena_slice_size.h");
            let mimalloc_include_path = std::env::current_dir().unwrap().join("mimalloc/include");
            let types_header_path = mimalloc_include_path.join("mimalloc/types.h");
            let header_content = format!(r#"#include "{}"

#ifdef MI_ARENA_SLICE_SIZE
#undef MI_ARENA_SLICE_SIZE
#endif

#define MI_ARENA_SLICE_SIZE ({})"#, types_header_path.display(), arena_slice_size);
            std::fs::write(&header_path, header_content).unwrap();
            let flag = format!("-I{} -include {}", mimalloc_include_path.display(), header_path.display());
            if cfg!(feature = "use_cxx") {
                cfg.cxxflag(flag);
            } else {
                cfg.cflag(flag);
            }
        }
    }

    println!("cargo:rustc-link-search=native={}/lib/mimalloc-3.1", cfg.build().display());

    let mut lib = "mimalloc".to_owned();
    if cfg!(feature = "secure") {
        lib += "-secure";
    }
    if cfg!(debug_assertions) {
        lib += "-debug";
    }

    println!("cargo:rustc-link-lib={}={}", link_type, lib);
}
