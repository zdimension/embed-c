mod reconstruct;

use std::fs;
use std::path::PathBuf;
use c2rust_transpile::{ReplaceMode, TranspilerConfig};
use proc_macro2::TokenStream;

extern crate proc_macro;

extern crate c2rust_transpile;

#[proc_macro]
pub fn embed_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    use crate::reconstruct::reconstruct;
    let code = reconstruct(TokenStream::from(input));

    use tempfile::{Builder, tempdir};
    let dir = tempdir().unwrap();
    let mut input_c = Builder::new()
        .suffix(".c")
        .tempfile_in(&dir)
        .unwrap();

    use std::io::{Write};

    writeln!(input_c, "{}", code).unwrap();

    let mut json_config = Builder::new()
        .suffix(".json")
        .tempfile_in(&dir)
        .unwrap();

    let json_code = format!(r#"[{{ "directory": "{}", "file": "{}" }} ]"#,
                            dir.path().display(), input_c.path().file_name().unwrap().to_str().unwrap());
    writeln!(json_config, "{}", json_code).unwrap();

    c2rust_transpile::transpile(TranspilerConfig {
        dump_untyped_context: false,
        dump_typed_context: false,
        pretty_typed_context: false,
        dump_function_cfgs: false,
        json_function_cfgs: false,
        dump_cfg_liveness: false,
        dump_structures: false,
        verbose: false,
        debug_ast_exporter: false,
        incremental_relooper: false,
        fail_on_multiple: false,
        filter: None,
        debug_relooper_labels: false,
        prefix_function_names: None,
        translate_asm: false,
        use_c_loop_info: false,
        use_c_multiple_info: false,
        simplify_structures: false,
        panic_on_translator_failure: false,
        emit_modules: false,
        fail_on_error: false,
        replace_unsupported_decls: ReplaceMode::None,
        translate_valist: false,
        overwrite_existing: false,
        reduce_type_annotations: false,
        reorganize_definitions: false,
        enabled_warnings: Default::default(),
        emit_no_std: false,
        output_dir: None,
        translate_const_macros: false,
        translate_fn_macros: false,
        disable_refactoring: false,
        preserve_unused_functions: false,
        log_level: log::LevelFilter::Off,
        emit_build_files: false,

        binaries: vec![]
    }, json_config.path(), &[""]);

    let mut converted_path = PathBuf::from(input_c.path());
    converted_path.set_extension("rs");
    let converted_code = fs::read_to_string(converted_path).unwrap();

    // Remove inner attributes
    use regex::Regex;
    let re = Regex::new(r"#!\[([^]])*]").unwrap();
    let final_code = re.replace_all(&converted_code, "");

    final_code.parse().unwrap()
}

