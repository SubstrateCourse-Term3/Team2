
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/Users/tingalin/Desktop/substrate-kitties/substrate/substrate-kitties/target/release/build/substrate-kitties-runtime-b593540f923ab877/out/wasm_binary.rs",
						"/Users/tingalin/Desktop/substrate-kitties/substrate/substrate-kitties/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base",
					)
				}
			