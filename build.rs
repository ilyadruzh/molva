extern crate vergen;

use vergen::{ConstantsFlags, Vergen};

const ERROR_MSG: &'static str = "Failed to generate metadata files";

fn main() {
	let vergen = Vergen::new(ConstantsFlags::all()).expect(ERROR_MSG);

	for (k, v) in vergen.build_info() {
		println!("cargo:rustc-env={}={}", k.name(), v);
	}

	println!("cargo:rerun-if-changed=.git/HEAD");
}
