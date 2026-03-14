savedcmd_module.o := OBJTREE=/usr/src/linux-6.18.12-gentoo RUST_MODFILE=./veles rustc --edition=2021 -Zbinary_dep_depinfo=y -Astable_features -Dnon_ascii_idents -Dunsafe_op_in_unsafe_fn -Wmissing_docs -Wrust_2018_idioms -Wunreachable_pub -Wclippy::all -Wclippy::as_ptr_cast_mut -Wclippy::as_underscore -Wclippy::cast_lossless -Wclippy::ignored_unit_patterns -Wclippy::mut_mut -Wclippy::needless_bitwise_bool -Aclippy::needless_lifetimes -Wclippy::no_mangle_with_rust_abi -Wclippy::ptr_as_ptr -Wclippy::ptr_cast_constness -Wclippy::ref_as_ptr -Wclippy::undocumented_unsafe_blocks -Wclippy::unnecessary_safety_comment -Wclippy::unnecessary_safety_doc -Wrustdoc::missing_crate_level_docs -Wrustdoc::unescaped_backticks -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Wclippy::float_arithmetic --target=/usr/src/linux-6.18.12-gentoo/scripts/target.json -Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2 -Zcf-protection=branch -Cjump-tables=n -Ctarget-cpu=x86-64 -Ztune-cpu=generic -Cno-redzone=y -Ccode-model=kernel -Zfunction-return=thunk-extern -Zpatchable-function-entry=16,16 -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cdebuginfo=2 -Zdebuginfo-compression=zlib  --cfg MODULE  @/usr/src/linux-6.18.12-gentoo/include/generated/rustc_cfg -Zallow-features=asm_const,asm_goto,arbitrary_self_types,lint_reasons,offset_of_nested,raw_ref_op,used_with_arg -Zcrate-attr=no_std -Zcrate-attr='feature(asm_const,asm_goto,arbitrary_self_types,lint_reasons,offset_of_nested,raw_ref_op,used_with_arg)' -Zunstable-options --extern pin_init --extern kernel --crate-type rlib -L /usr/src/linux-6.18.12-gentoo/rust/ --crate-name module --sysroot=/dev/null --out-dir ./ --emit=dep-info=./.module.o.d --emit=obj=module.o module.rs 

source_module.o := module.rs

deps_module.o := \
  /usr/src/linux-6.18.12-gentoo/rust/libcore.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libkernel.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libffi.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libcompiler_builtins.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libpin_init.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libmacros.so \
  /usr/src/linux-6.18.12-gentoo/rust/libpin_init_internal.so \
  /usr/src/linux-6.18.12-gentoo/rust/libbuild_error.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libbindings.rmeta \
  /usr/src/linux-6.18.12-gentoo/rust/libuapi.rmeta \

module.o: $(deps_module.o)

$(deps_module.o):
