# 使用buck2创建Rust的`hello world`



## 软件简介

几天前，`Facebook/Meta/idk`宣布`buck2`现在已经开源。`buck2` 是一个用 Rust 编写的大型高性能构建系统，是 `buck1` 构建系统的继承者。`Buck2`是一个用Rust编写的可扩展和高性能的构建系统，旨在使你的构建体验更快、更高效。

## 安装

`buck2` 是用 Rust 编写的，它需要 `rustup` 来编译。

直接从 GitHub 上安装它：

安装环境必须是nightly-2023-01-24版本，其它版本编译有可能出问题。

```shell
$ rustup install nightly-2023-01-24
$ cargo +nightly-2023-01-24 install --git https://github.com/facebook/buck2.git buck2
```

安装时间有电长请耐心等待。作为一个Rust用户，这种安装对我来说是非常方便的，但如果你没有安装Rust，需要学习一下。

## 入门

[官方给了一个简单的入门指南](https://buck2.build/docs/getting_started/)，不过介绍的主要是`c++`的使用方式。本文主要以，我需要的Rust进行举例。



## 创建第一个工程

```shell
$ mkdir buck2-hello && cd buck2-hello
```

要初始化这个项目，我们使用这个命令：

```shell
$ buck2 init --git
```

在我们继续下面步骤之前，让我们研究一下这对我们产生了什么影响。

### 项目目录的初始情况

```shell
$ git add .
$ git status
On branch main

No commits yet

Changes to be committed:
  (use "git rm --cached <file>..." to unstage)
        new file:   .buckconfig
        new file:   .gitignore
        new file:   .gitmodules
        new file:   BUCK
        new file:   prelude
        new file:   toolchains/BUCK
$ tree
.
├── BUCK
├── prelude
│   ├── alias.bzl
│   ├── android
│   │   ├── aapt2_link.bzl
│   │   ├── android_aar.bzl
│   │   ├── android_apk.bzl
│   │   ├── android_binary_native_library_rules.bzl
│   │   ├── android_binary_resources_rules.bzl
│   │   ├── android_build_config.bzl
│   │   ├── android.bzl
│   │   ├── android_instrumentation_apk.bzl
│   │   ├── android_instrumentation_test.bzl
│   │   ├── android_library.bzl
│   │   ├── android_manifest.bzl
│   │   ├── android_prebuilt_aar.bzl
│   │   ├── android_providers.bzl
│   │   ├── android_resource.bzl
│   │   ├── android_toolchain.bzl
│   │   ├── apk_genrule.bzl
│   │   ├── configuration.bzl
│   │   ├── cpu_filters.bzl
│   │   ├── dex_rules.bzl
│   │   ├── exopackage.bzl
│   │   ├── gen_aidl.bzl
│   │   ├── min_sdk_version.bzl
│   │   ├── prebuilt_native_library.bzl
│   │   ├── preprocess_java_classes.bzl
│   │   ├── proguard.bzl
│   │   ├── r_dot_java.bzl
│   │   ├── robolectric_test.bzl
│   │   └── voltron.bzl
│   ├── apple
│   │   ├── apple_asset_catalog.bzl
│   │   ├── apple_asset_catalog_compilation_options.bzl
│   │   ├── apple_asset_catalog_types.bzl
│   │   ├── apple_binary.bzl
│   │   ├── apple_bundle.bzl
│   │   ├── apple_bundle_config.bzl
│   │   ├── apple_bundle_destination.bzl
│   │   ├── apple_bundle_macro_layer.bzl
│   │   ├── apple_bundle_part.bzl
│   │   ├── apple_bundle_resources.bzl
│   │   ├── apple_bundle_types.bzl
│   │   ├── apple_bundle_utility.bzl
│   │   ├── apple_code_signing_types.bzl
│   │   ├── apple_core_data.bzl
│   │   ├── apple_core_data_types.bzl
│   │   ├── apple_dsym.bzl
│   │   ├── apple_frameworks.bzl
│   │   ├── apple_framework_versions.bzl
│   │   ├── apple_info_plist.bzl
│   │   ├── apple_info_plist_substitutions_parsing.bzl
│   │   ├── apple_library.bzl
│   │   ├── apple_macro_layer.bzl
│   │   ├── apple_modular_utility.bzl
│   │   ├── apple_package.bzl
│   │   ├── apple_package_config.bzl
│   │   ├── apple_resource.bzl
│   │   ├── apple_resource_types.bzl
│   │   ├── apple_resource_utility.bzl
│   │   ├── apple_rules_impl.bzl
│   │   ├── apple_rules_impl_utility.bzl
│   │   ├── apple_sdk.bzl
│   │   ├── apple_sdk_metadata.bzl
│   │   ├── apple_stripping.bzl
│   │   ├── apple_target_sdk_version.bzl
│   │   ├── apple_test.bzl
│   │   ├── apple_test_macro_layer.bzl
│   │   ├── apple_toolchain.bzl
│   │   ├── apple_toolchain_types.bzl
│   │   ├── apple_utility.bzl
│   │   ├── modulemap.bzl
│   │   ├── prebuilt_apple_framework.bzl
│   │   ├── resource_groups.bzl
│   │   ├── swift
│   │   │   ├── apple_sdk_clang_module.bzl
│   │   │   ├── apple_sdk_modules_utility.bzl
│   │   │   ├── apple_sdk_swift_module.bzl
│   │   │   ├── swift_compilation.bzl
│   │   │   ├── swift_module_map.bzl
│   │   │   ├── swift_pcm_compilation.bzl
│   │   │   ├── swift_pcm_compilation_types.bzl
│   │   │   ├── swift_sdk_pcm_compilation.bzl
│   │   │   ├── swift_sdk_swiftinterface_compilation.bzl
│   │   │   ├── swift_toolchain.bzl
│   │   │   └── swift_toolchain_types.bzl
│   │   ├── tools
│   │   │   ├── BUCK
│   │   │   ├── make_modulemap.py
│   │   │   ├── make_vfsoverlay.py
│   │   │   ├── swift_exec.sh
│   │   │   └── swift_objc_header_postprocess.py
│   │   ├── user
│   │   │   ├── apple_focused_debugging.bzl
│   │   │   ├── apple_resource_bundle.bzl
│   │   │   ├── apple_toolchain_override.bzl
│   │   │   ├── apple_tools.bzl
│   │   │   ├── apple_watchos_bundle.bzl
│   │   │   ├── resource_group_map.bzl
│   │   │   └── watch_transition.bzl
│   │   ├── xcode.bzl
│   │   ├── xcode_postbuild_script.bzl
│   │   └── xcode_prebuild_script.bzl
│   ├── asserts.bzl
│   ├── attributes.bzl
│   ├── BUCK
│   ├── builtin.bzl
│   ├── cache_mode.bzl
│   ├── CHANGELOG.md
│   ├── CODE_OF_CONDUCT.md
│   ├── command_alias.bzl
│   ├── configurations
│   │   ├── rules.bzl
│   │   └── util.bzl
│   ├── CONTRIBUTING.md
│   ├── cpu
│   │   ├── BUCK
│   │   └── constraints
│   │       └── BUCK
│   ├── cxx
│   │   ├── archive.bzl
│   │   ├── attr_selection.bzl
│   │   ├── comp_db.bzl
│   │   ├── compile.bzl
│   │   ├── compiler.bzl
│   │   ├── cxx_bolt.bzl
│   │   ├── cxx.bzl
│   │   ├── cxx_context.bzl
│   │   ├── cxx_executable.bzl
│   │   ├── cxx_library.bzl
│   │   ├── cxx_library_utility.bzl
│   │   ├── cxx_link_utility.bzl
│   │   ├── cxx_toolchain.bzl
│   │   ├── cxx_toolchain_types.bzl
│   │   ├── cxx_types.bzl
│   │   ├── debug.bzl
│   │   ├── dist_lto
│   │   │   ├── dist_lto.bzl
│   │   │   ├── README.md
│   │   │   ├── tools
│   │   │   │   ├── BUCK
│   │   │   │   ├── dist_lto_copy.py
│   │   │   │   ├── dist_lto_opt.py
│   │   │   │   ├── dist_lto_planner.py
│   │   │   │   ├── dist_lto_prepare.py
│   │   │   │   ├── __init__.py
│   │   │   │   └── tests
│   │   │   │       └── test_dist_lto_opt.py
│   │   │   └── tools.bzl
│   │   ├── dwp.bzl
│   │   ├── groups.bzl
│   │   ├── headers.bzl
│   │   ├── link.bzl
│   │   ├── linker.bzl
│   │   ├── link_groups.bzl
│   │   ├── omnibus.bzl
│   │   ├── platform.bzl
│   │   ├── prebuilt_cxx_library_group.bzl
│   │   ├── preprocessor.bzl
│   │   ├── symbols.bzl
│   │   ├── tools
│   │   │   ├── BUCK
│   │   │   ├── defs.bzl
│   │   │   ├── dep_file_processor.py
│   │   │   ├── dep_file_utils.py
│   │   │   ├── make_comp_db.py
│   │   │   ├── makefile_to_dep_file.py
│   │   │   ├── show_headers_to_dep_file.py
│   │   │   └── show_includes_to_dep_file.py
│   │   ├── user
│   │   │   ├── cxx_toolchain_override.bzl
│   │   │   └── link_group_map.bzl
│   │   └── xcode.bzl
│   ├── decls
│   │   ├── android_common.bzl
│   │   ├── android_rules.bzl
│   │   ├── apple_common.bzl
│   │   ├── common.bzl
│   │   ├── core_rules.bzl
│   │   ├── cxx_common.bzl
│   │   ├── cxx_rules.bzl
│   │   ├── d_common.bzl
│   │   ├── dotnet_rules.bzl
│   │   ├── d_rules.bzl
│   │   ├── genrule_common.bzl
│   │   ├── go_common.bzl
│   │   ├── go_rules.bzl
│   │   ├── groovy_rules.bzl
│   │   ├── halide_rules.bzl
│   │   ├── haskell_common.bzl
│   │   ├── haskell_rules.bzl
│   │   ├── ios_rules.bzl
│   │   ├── java_rules.bzl
│   │   ├── js_rules.bzl
│   │   ├── jvm_common.bzl
│   │   ├── kotlin_rules.bzl
│   │   ├── lua_common.bzl
│   │   ├── lua_rules.bzl
│   │   ├── native_common.bzl
│   │   ├── ocaml_common.bzl
│   │   ├── ocaml_rules.bzl
│   │   ├── python_common.bzl
│   │   ├── python_rules.bzl
│   │   ├── remote_common.bzl
│   │   ├── rust_common.bzl
│   │   ├── rust_rules.bzl
│   │   ├── scala_rules.bzl
│   │   ├── shell_rules.bzl
│   │   ├── toolchains_common.bzl
│   │   └── uncategorized_rules.bzl
│   ├── defs.bzl
│   ├── docs
│   │   └── rules.bzl
│   ├── erlang
│   │   ├── applications
│   │   │   └── BUCK
│   │   ├── common_test
│   │   │   ├── common
│   │   │   │   ├── BUCK
│   │   │   │   ├── include
│   │   │   │   │   ├── artifact_annotations.hrl
│   │   │   │   │   ├── buck_ct_records.hrl
│   │   │   │   │   └── tpx_records.hrl
│   │   │   │   └── src
│   │   │   │       ├── artifact_annotations.erl
│   │   │   │       ├── bounded_buffer.erl
│   │   │   │       ├── buck_ct_parser.erl
│   │   │   │       ├── buck_ct_provider.erl
│   │   │   │       ├── ct_error_printer.erl
│   │   │   │       ├── execution_logs.erl
│   │   │   │       ├── io_buffer.erl
│   │   │   │       ├── test_artifact_directory.erl
│   │   │   │       └── test_logger.erl
│   │   │   ├── cth_hooks
│   │   │   │   ├── BUCK
│   │   │   │   └── src
│   │   │   │       └── cth_tpx.erl
│   │   │   ├── test_binary
│   │   │   │   ├── BUCK
│   │   │   │   └── src
│   │   │   │       ├── json_interfacer.erl
│   │   │   │       ├── junit_interfacer.erl
│   │   │   │       ├── listing_interfacer.erl
│   │   │   │       ├── list_test.erl
│   │   │   │       ├── test_binary.erl
│   │   │   │       └── test_runner.erl
│   │   │   ├── test_cli_lib
│   │   │   │   ├── BUCK
│   │   │   │   └── src
│   │   │   │       └── test.erl
│   │   │   └── test_exec
│   │   │       ├── BUCK
│   │   │       └── src
│   │   │           ├── ct_daemon_core.erl
│   │   │           ├── ct_daemon.erl
│   │   │           ├── ct_daemon_hooks.erl
│   │   │           ├── ct_daemon_logger.erl
│   │   │           ├── ct_daemon_node.erl
│   │   │           ├── ct_daemon_printer.erl
│   │   │           ├── ct_daemon_runner.erl
│   │   │           ├── ct_executor.erl
│   │   │           ├── ct_runner.erl
│   │   │           ├── epmd_manager.erl
│   │   │           ├── test_exec.app.src
│   │   │           ├── test_exec.erl
│   │   │           └── test_exec_sup.erl
│   │   ├── erlang_application.bzl
│   │   ├── erlang_application_includes.bzl
│   │   ├── erlang_build.bzl
│   │   ├── erlang.bzl
│   │   ├── erlang_dependencies.bzl
│   │   ├── erlang_escript.bzl
│   │   ├── erlang_info.bzl
│   │   ├── erlang_ls.config
│   │   ├── erlang_otp_application.bzl
│   │   ├── erlang_release.bzl
│   │   ├── erlang_shell.bzl
│   │   ├── erlang_tests.bzl
│   │   ├── erlang_toolchain.bzl
│   │   ├── erlang_utils.bzl
│   │   ├── shell
│   │   │   ├── BUCK
│   │   │   ├── shell.bxl
│   │   │   └── src
│   │   │       ├── shell_buck2_utils.erl
│   │   │       └── user_default.erl
│   │   └── toolchain
│   │       ├── app_src_builder.escript
│   │       ├── boot_script_builder.escript
│   │       ├── BUCK
│   │       ├── dependency_analyzer.escript
│   │       ├── edoc_cli.escript
│   │       ├── edoc_doclet_chunks.erl
│   │       ├── edoc_report.erl
│   │       ├── erlang_ls.config
│   │       ├── erlc_trampoline.sh
│   │       ├── escript_builder.escript
│   │       ├── release_variables_builder.escript
│   │       └── transform_project_root.erl
│   ├── export_file.bzl
│   ├── filegroup.bzl
│   ├── force_hybrid_links.bzl
│   ├── genrule.bzl
│   ├── genrule_local_labels.bzl
│   ├── genrule_toolchain.bzl
│   ├── go
│   │   ├── cgo_library.bzl
│   │   ├── compile.bzl
│   │   ├── coverage.bzl
│   │   ├── go_binary.bzl
│   │   ├── go_library.bzl
│   │   ├── go_test.bzl
│   │   ├── link.bzl
│   │   ├── packages.bzl
│   │   ├── toolchain.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       ├── cgo_wrapper.py
│   │       ├── compile_wrapper.py
│   │       ├── cover_srcs.py
│   │       ├── filter_srcs.py
│   │       └── testmaingen.go
│   ├── haskell
│   │   ├── haskell.bzl
│   │   ├── haskell_ghci.bzl
│   │   ├── haskell_haddock.bzl
│   │   └── haskell_ide.bzl
│   ├── http_archive
│   │   ├── http_archive.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       └── create_exclusion_list.py
│   ├── http_file.bzl
│   ├── ide_integrations
│   │   └── xcode.bzl
│   ├── java
│   │   ├── class_to_srcs.bzl
│   │   ├── dex.bzl
│   │   ├── dex_toolchain.bzl
│   │   ├── jar_genrule.bzl
│   │   ├── java_binary.bzl
│   │   ├── java.bzl
│   │   ├── javacd_jar_creator.bzl
│   │   ├── java_library.bzl
│   │   ├── java_providers.bzl
│   │   ├── java_resources.bzl
│   │   ├── java_test.bzl
│   │   ├── java_toolchain.bzl
│   │   ├── keystore.bzl
│   │   ├── plugins
│   │   │   ├── java_annotation_processor.bzl
│   │   │   └── java_plugin.bzl
│   │   ├── prebuilt_jar.bzl
│   │   ├── tools
│   │   │   ├── BUCK
│   │   │   ├── gen_class_to_source_map.py
│   │   │   ├── merge_class_to_source_maps.py
│   │   │   └── used_classes_to_dep_file.py
│   │   └── utils
│   │       └── java_utils.bzl
│   ├── js
│   │   ├── js_bundle.bzl
│   │   ├── js_bundle_genrule.bzl
│   │   ├── js.bzl
│   │   ├── js_library.bzl
│   │   ├── js_providers.bzl
│   │   └── js_utils.bzl
│   ├── julia
│   │   ├── julia_binary.bzl
│   │   ├── julia.bzl
│   │   ├── julia_info.bzl
│   │   ├── julia_library.bzl
│   │   ├── julia_test.bzl
│   │   ├── julia_toolchain.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       └── parse_julia_cmd.py
│   ├── jvm
│   │   └── cd_jar_creator_util.bzl
│   ├── kotlin
│   │   ├── kotlin.bzl
│   │   ├── kotlincd_jar_creator.bzl
│   │   ├── kotlin_library.bzl
│   │   ├── kotlin_test.bzl
│   │   ├── kotlin_toolchain.bzl
│   │   └── kotlin_utils.bzl
│   ├── LICENSE-APACHE
│   ├── LICENSE-MIT
│   ├── linking
│   │   ├── linkable_graph.bzl
│   │   ├── linkables.bzl
│   │   ├── link_groups.bzl
│   │   ├── link_info.bzl
│   │   ├── lto.bzl
│   │   ├── shared_libraries.bzl
│   │   └── strip.bzl
│   ├── local_only.bzl
│   ├── lua
│   │   ├── cxx_lua_extension.bzl
│   │   ├── lua_binary.bzl
│   │   └── lua_library.bzl
│   ├── native.bzl
│   ├── ocaml
│   │   ├── attrs.bzl
│   │   ├── makefile.bzl
│   │   ├── ocaml.bzl
│   │   └── ocaml_toolchain_types.bzl
│   ├── os
│   │   ├── BUCK
│   │   └── constraints
│   │       └── BUCK
│   ├── os_lookup
│   │   ├── defs.bzl
│   │   └── targets
│   │       └── BUCK
│   ├── paths.bzl
│   ├── platforms
│   │   ├── BUCK
│   │   └── defs.bzl
│   ├── playground
│   │   └── test.bxl
│   ├── prelude.bzl
│   ├── pull_request_template.md
│   ├── python
│   │   ├── compile.bzl
│   │   ├── cxx_python_extension.bzl
│   │   ├── interface.bzl
│   │   ├── make_pex.bzl
│   │   ├── manifest.bzl
│   │   ├── native_python_util.bzl
│   │   ├── needed_coverage.bzl
│   │   ├── prebuilt_python_library.bzl
│   │   ├── python_binary.bzl
│   │   ├── python.bzl
│   │   ├── python_library.bzl
│   │   ├── python_needed_coverage_test.bzl
│   │   ├── python_test.bzl
│   │   ├── sourcedb
│   │   │   ├── build.bxl
│   │   │   ├── classic.bxl
│   │   │   ├── code_navigation.bxl
│   │   │   ├── merge.bxl
│   │   │   └── query.bxl
│   │   ├── source_db.bzl
│   │   ├── toolchain.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       ├── compile.py
│   │       ├── create_manifest_for_source_dir.py
│   │       ├── embedded_main.cpp
│   │       ├── extract.py
│   │       ├── fail_with_message.py
│   │       ├── generate_static_extension_info.py
│   │       ├── make_pex_inplace.py
│   │       ├── make_pex_manifest_module.py
│   │       ├── make_pex_modules.py
│   │       ├── make_source_db_no_deps.py
│   │       ├── make_source_db.py
│   │       ├── parse_imports.py
│   │       ├── py38stdlib.py
│   │       ├── run_inplace_lite.py.in
│   │       ├── run_inplace.py.in
│   │       ├── sourcedb_merger
│   │       │   ├── BUCK
│   │       │   ├── inputs.py
│   │       │   ├── legacy_merge.py
│   │       │   ├── legacy_outputs.py
│   │       │   ├── merge.py
│   │       │   ├── outputs.py
│   │       │   └── tests
│   │       │       ├── __init__.py
│   │       │       ├── inputs_test.py
│   │       │       ├── legacy_output_test.py
│   │       │       ├── main.sh
│   │       │       └── outputs_test.py
│   │       ├── static_extension_finder.py
│   │       ├── static_extension_utils.cpp
│   │       ├── __test_main__.py
│   │       └── traverse_dep_manifest.py
│   ├── python_bootstrap
│   │   ├── python_bootstrap.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       └── win_python_wrapper.bat
│   ├── README.md
│   ├── remote_file.bzl
│   ├── resources.bzl
│   ├── rules.bzl
│   ├── rules_impl.bzl
│   ├── rust
│   │   ├── build.bzl
│   │   ├── build_params.bzl
│   │   ├── context.bzl
│   │   ├── extern.bzl
│   │   ├── failure_filter.bzl
│   │   ├── link_info.bzl
│   │   ├── resources.bzl
│   │   ├── rust-analyzer
│   │   │   └── resolve_deps.bxl
│   │   ├── rust_binary.bzl
│   │   ├── rust_library.bzl
│   │   ├── rust_toolchain.bzl
│   │   ├── targets.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       ├── concat.py
│   │       ├── failure_filter_action.py
│   │       ├── rustc_action.py
│   │       ├── rustdoc_test_with_resources.py
│   │       └── transitive_dependency_symlinks.py
│   ├── sh_binary.bzl
│   ├── sh_test.bzl
│   ├── test
│   │   ├── inject_test_run_info.bzl
│   │   └── tools
│   │       ├── BUCK
│   │       └── inject_test_env.py
│   ├── tests
│   │   ├── re_utils.bzl
│   │   └── tpx_re_legacy.bzl
│   ├── test_suite.bzl
│   ├── toolchains
│   │   ├── cxx
│   │   │   └── zig
│   │   │       ├── defs.bzl
│   │   │       └── releases.bzl
│   │   ├── cxx.bzl
│   │   ├── genrule.bzl
│   │   ├── ocaml.bzl
│   │   ├── python.bzl
│   │   └── rust.bzl
│   ├── tools
│   │   ├── BUCK
│   │   └── find_and_replace.bat
│   ├── transitions
│   │   └── constraint_overrides.bzl
│   ├── user
│   │   ├── all.bzl
│   │   ├── extract_archive.bzl
│   │   └── rule_spec.bzl
│   ├── utils
│   │   ├── build_target_pattern.bzl
│   │   ├── dicts.bzl
│   │   ├── graph_utils.bzl
│   │   ├── pick.bzl
│   │   ├── platform_flavors_util.bzl
│   │   ├── set.bzl
│   │   ├── types.bzl
│   │   └── utils.bzl
│   ├── windows
│   │   └── tools
│   │       ├── BUCK
│   │       └── msvc_hermetic_exec.bat
│   ├── worker_tool.bzl
│   └── zip_file
│       ├── tools
│       │   ├── BUCK
│       │   └── unzip.py
│       ├── zip_file.bzl
│       └── zip_file_toolchain.bzl
└── toolchains
    └── BUCK

83 directories, 480 files


```

让我们详细讲解一下这些文件。

### `.buckconfig`

```toml
[repositories]
root = .
prelude = prelude
toolchains = toolchains
none = none

[repository_aliases]
config = prelude
fbcode = none
fbsource = none
buck = none

[parser]
target_platform_detector_spec = target:root//...->prelude//platforms:default
```

none = none的这种写法有点好笑。不管怎么说，这个文件是非常重要的：它配置了整个工程。在某种意义上，它就像`Cargo.toml`：就像一个包是由`Cargo.toml`所定义的，一个`.buckconfig`定义了一个`xxx`单元的存在，它定义了一个包。重点是，这是一个很高级的配置。项目的根目录在当前目录下，我们想使用默认的`prelude`和工具链。

> 我不知道none = none是做什么用的，因为我还没有用到。现在的官网文档也很简单，等以后有了新的资料，我再补充。



### `.gitmodules` & `prelude`

还有一个‵.gitmodules`文件，指向https://github.com/facebook/buck2-prelude.git 这个地址，下载的文件在prelude目录下。如果你在那里查看，你会发现一堆.bzl文件，实现了一些有用的功能供我们使用。我们稍后会讨论这些，但重点是，这有点像一个 "标准库"，如果你愿意的话，你也可以不使用它而定义你自己的，如果你有这个本事的话。

### BUCK

现在我们来看一下`BUCK`文件：

```
# A list of available rules and their signatures can be found here: https://buck2.build/docs/api/rules/

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)
```

‵genrule‵就像一个函数，由我们的prelude提供。如果你感到好奇，其实现是在‵prelude/genrule.bzl‵中。这个命令，正如你所想象的，设置了一个名为‵hello_world‵的规则，产生一个名为‵out.txt‵的文件。它通过运行‵cmd‵来完成这个任务。很好，很简单明了。



### `toolchains/BUCK`

这个文件描述了一个工具链。内容如下：

```perl
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")

system_genrule_toolchain(
    name = "genrule",
    visibility = ["PUBLIC"],
)
```

这是从prelude中加载某个规则，然后将其定义为一个公共工具链。我们可以在这里定义任意多的工具链，例如，如果我们想同时构建Rust和Python，我们可以在这里定义这两个工具链供以后使用。

`genrule`工具链用于从shell命令中生成文件，正如我们之前看到的产生`out.txt`的规则。所以，在我的理解中，我们在这里定义了我们要如何使用它。然后，在BUCK文件中，我们正在使用这个工具链来实现我们的规则。

## 对我们的第一个工程进行调试

现在，让我们真正地试一试。为了指示Buck如何建立，我们使用 "目标模式 "作为参数来操作。让我们看看Buck它知道如何建立什么目标。如下：

```shell
$ buck2 targets //...
Build ID: 2c955f2d-1785-4906-beda-ff9bb8608246
Jobs completed: 4. Time elapsed: 0.0s.
root//:hello_world
```

//...是一个 "目标模式"。/...表示 "子目录中的所有构建文件中的构建目标"，而/表示我们的根目录，所以//...表示 "所有子目录中的所有构建文件的所有目标"。通过将这个目标传递给 buck2 targets，我们可以看到项目中的每一个目标。这显示了我们定义的一个目标，root://:hello_world。这个名字是在我们上面的BUCK文件中定义的。如果你把它改成如下:

```perl
# A list of available rules and their signatures can be found here: https://buck2.build/docs/api/rules/

genrule(
    name = "buck2_hello",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)
```

执行上面的命令：

```shell
$ buck2 targets //...
Build ID: cbd7e1b2-297a-4cb8-ab4a-4f44006d50b1
Jobs completed: 2. Time elapsed: 0.0s.
root//:buck2_hello
```

现在我们继续：

```shell
$ buck2 build //:buck2_hello
File changed: root//BUCK
Build ID: 46baf54c-282f-46f8-ae6e-9e7cc868d6e0
Jobs completed: 9. Time elapsed: 0.0s. Cache hits: 0%. Commands: 1 (cached: 0, remote: 0, local: 1)
BUILD SUCCEEDED
```

OK,构建成功了，但我们的`out.txt`在哪里？我们可以问buck!

```shell
$ buck2 build //:buck2_hello --show-output
Build ID: 8b9f9178-5cfb-4baa-b6fb-44ead2027899
Jobs completed: 3. Time elapsed: 0.0s.
BUILD SUCCEEDED
root//:buck2_hello buck-out/v2/gen/root/524f8da68ea2a374/__buck2_hello__/out/out.txt
```

这个文件在buck-out的一个深度嵌套的子目录中，这是一个为我们创建的新的顶级目录。

如果我们看一下这个文件，你可以看到它包含了我们希望它包含的文本。

```shell
$ cat buck-out/v2/gen/root/524f8da68ea2a374/__buck2_hello__/out/out.txt
BUILT BY BUCK2
```

这个时候我们看一下`.gitignore`文件

```shell
$ cat .gitignore 
/buck-out
```

这个`buck-out`是被忽略的，我们现在删掉这一行，取消忽略。

让我们进行第二次构建：

```shell
 buck2 build //:buck2_hello
File changed: root//.gitignore.swp
File changed: root//.gitignore.swx
File changed: root//4913
2 additional file change events
Build ID: 3ee029db-fe60-41e9-aa45-2a2260ea70db
Jobs completed: 3. Time elapsed: 0.0s.
BUILD SUCCEEDED
```

buck注意到我们修改了一些文件，但由于我们的规则并不依赖于任何文件，所以我们可以正常使用。

### 编写rust代码

现在我们创建一个rust文件`main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

同时，更新一下BUCK文件,添加下面：

```perl
rust_binary(
    name = "buck2_hello",
    srcs = ["main.rs"],
    crate_root = "hello.rs",
)
```

编译：

```shell
$ buck2 build //:hello_world
File changed: root//.BUCK.swp
File changed: root//.BUCK.swpx
File changed: root//4913
2 additional file change events
Error running analysis for `root//:hello_world (prelude//platforms:default#524f8da68ea2a374)`

Caused by:
    0: Error looking up configured node root//:hello_world (prelude//platforms:default#524f8da68ea2a374)
    1: Error looking up configured node toolchains//:cxx (prelude//platforms:default#524f8da68ea2a374) (prelude//platforms:default#524f8da68ea2a374)
    2: looking up unconfigured target node `toolchains//:cxx`
    3: Unknown target `cxx` from package `toolchains//`.
       Did you mean one of the 2 targets in toolchains//:BUCK?
       Maybe you meant one of these similar targets?
         toolchains//:rust
Build ID: 71ab162a-4de0-4b98-89d9-9b391d74b322
Jobs completed: 8. Time elapsed: 0.0s.
BUILD FAILED
```

Oops!我们没有建立一个Rust工具链!让我们现在来做。编辑`toolchains/BUCK`：

```perl
load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")

system_rust_toolchain(
    name = "rust",
    default_edition = "2021",
    visibility = ["PUBLIC"],
)
```

再次编译，错误依然！

```
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")
load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")

system_genrule_toolchain(
    name = "genrule",
    visibility = ["PUBLIC"],
)

system_rust_toolchain(
    name = "rust",
    default_edition = "2021",
    visibility = ["PUBLIC"],
)

```







## 引用



[地址](https://steveklabnik.com/writing/using-buck-to-build-rust-projects)
