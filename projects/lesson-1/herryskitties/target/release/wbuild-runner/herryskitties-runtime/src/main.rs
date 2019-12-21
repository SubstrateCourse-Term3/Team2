
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/root/substrate/herryskitties/target/release/build/herryskitties-runtime-e6995ec80ffc25ea/out/wasm_binary.rs",
						"/root/substrate/herryskitties/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base",
					)
				}
			