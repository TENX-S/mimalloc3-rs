
fn main() {
    #[cfg(all(feature = "static", feature = "shared"))]
    compile_error!("You have enabled both 'static' and 'shared' features, which conflicts. You must choose one linking method: 'static' for static linking, or 'shared' for dynamic linking.");

    #[cfg(not(any(feature = "static", feature = "shared")))]
    compile_error!("You must enable exactly one of the features: 'static' or 'shared'.");

    #[cfg(all(feature = "static", feature = "override", target_os = "windows"))]
    compile_error!("It is only possible to override malloc on Windows when building as a DLL.");

    let mut cfg = cmake::Config::new("mimalloc");

    let linkage: &str;

    #[cfg(feature = "static")]
    {
        cfg.define("MI_BUILD_STATIC", "ON");
        cfg.define("MI_BUILD_SHARED", "OFF");
        linkage = "static";
    }

    #[cfg(feature = "shared")]
    {
        cfg.define("MI_BUILD_STATIC", "OFF");
        cfg.define("MI_BUILD_SHARED", "ON");
        linkage = "dylib";
    }

    #[cfg(feature = "secure")]
    cfg.define("MI_SECURE", "ON");

    #[cfg(feature = "debug-full")]
    cfg.define("MI_DEBUG_FULL", "ON");

    #[cfg(feature = "padding")]
    cfg.define("MI_PADDING", "ON");

    #[cfg(feature = "override")]
    cfg.define("MI_OVERRIDE", "ON");

    #[cfg(feature = "xmalloc")]
    cfg.define("MI_XMALLOC", "ON");

    #[cfg(feature = "show_errors")]
    cfg.define("MI_SHOW_ERRORS", "ON");

    #[cfg(feature = "guarded")]
    cfg.define("MI_GUARDED", "ON");

    #[cfg(feature = "use_cxx")]
    cfg.define("MI_USE_CXX", "ON");

    #[cfg(feature = "opt_arch")]
    cfg.define("MI_OPT_ARCH", "ON");

    #[cfg(feature = "opt_simd")]
    cfg.define("MI_OPT_SIMD", "ON");

    #[cfg(feature = "see_asm")]
    cfg.define("MI_SEE_ASM", "ON");

    #[cfg(all(target_os = "macos", feature = "osx_interpose"))]
    {
        cfg.define("MI_OSX_INTERPOSE", "ON");
        #[cfg(feature = "use_cxx")]
        println!("cargo:warning=if dynamically overriding malloc/free, it is more reliable to build mimalloc as C code (don't enable feature use-cxx)")
    }

    #[cfg(all(target_os = "macos", feature = "osx_zone"))]
    cfg.define("MI_OSX_ZONE", "ON");

    #[cfg(all(target_os = "windows", feature = "win_redirect"))]
    cfg.define("MI_WIN_REDIRECT", "ON");

    #[cfg(all(target_os = "windows", feature = "win_use_fixed_tls"))]
    cfg.define("MI_WIN_USE_FIXED_TLS", "ON");

    #[cfg(all(target_family = "unix", feature = "local_dynamic_tls"))]
    cfg.define("MI_LOCAL_DYNAMIC_TLS", "ON");

    #[cfg(all(target_env = "musl"))]
    cfg.define("MI_LIBC_MUSL", "ON");

    #[cfg(feature = "debug_asan")]
    cfg.define("MI_DEBUG_ASAN", "ON");

    #[cfg(feature = "debug_ubsan")]
    cfg.define("MI_DEBUG_UBSAN", "ON");

    #[cfg(feature = "track_valgrind")]
    cfg.define("MI_TRACK_VALGRIND", "ON");

    #[cfg(feature = "track_asan")]
    cfg.define("MI_TRACK_ASAN", "ON");

    #[cfg(feature = "track_etw")]
    cfg.define("MI_TRACK_ETW", "ON");

    cfg.define("MI_BUILD_OBJECT", "OFF");
    #[cfg(feature = "build_object")]
    cfg.define("MI_BUILD_OBJECT", "ON");

    cfg.define("MI_BUILD_TESTS", "OFF");
    #[cfg(feature = "build_tests")]
    cfg.define("MI_BUILD_TESTS", "ON");

    #[cfg(feature = "skip_collect_on_exit")]
    cfg.define("MI_SKIP_COLLECT_ON_EXIT", "ON");

    #[cfg(feature = "no_padding")]
    cfg.define("MI_NO_PADDING", "ON");

    #[cfg(all(any(target_os = "linux", target_os = "android"), feature = "no_thp"))]
    cfg.define("MI_NO_THP", "ON");

    println!("cargo:rustc-link-search=native={}/lib/mimalloc-3.1", cfg.build().display());

    let mut lib = "mimalloc";
    if cfg!(debug_assertions) {
        lib = "mimalloc-debug";
    }

    println!("cargo:rustc-link-lib={}={}", linkage, lib);
}
