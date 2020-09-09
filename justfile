# Build with Profile Guided Optimizations
pgo:
	@echo "STEP 0: Make sure there is no left-over profiling data from previous runs"
	@rm -rf /tmp/pgo-data
	@cargo clean
	@cp examples/pq.rs src/main.rs
	@sed -i 's/\[dev-dependencies\]/\[dependencies\]/g' Cargo.toml

	@echo "STEP 1: Build the instrumented binaries"
	@env RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo +nightly build --release --target=x86_64-unknown-linux-gnu

	@echo "STEP 2: Run the instrumented binaries with some typical data"
	@for file in `seq 1 100`; do ./target/x86_64-unknown-linux-gnu/release/pquote; done

	@echo "STEP 3: Merge the '.profraw' files into a '.profdata' file"
	@llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

	@echo "STEP 4: Use the '.profdata' file for guiding optimizations"
	@RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -C link-arg=-s" cargo +nightly build --release --target=x86_64-unknown-linux-gnu
	@rm src/main.rs
	@sed -i 's/\[dependencies\]/\[dev-dependencies\]/g' Cargo.toml

# Build pq example
pq:
	cargo build --release --example pq

# Get size of binary
size:
	#!/usr/bin/bash
	if [[ -f target/x86_64-unknown-linux-gnu/release/examples/pq && -f target/release/examples/pq ]]; then
		echo "PGO:    " $(du -h target/x86_64-unknown-linux-gnu/release/examples/pq | awk '{print $1}')
		echo "Release:" $(du -h target/release/examples/pq | awk '{print $1}')
	else
		if [ -f target/x86_64-unknown-linux-gnu/release/examples/pq ]; then
			echo "PGO:" $(du -h target/x86_64-unknown-linux-gnu/release/examples/pq | awk '{print $1}')
		fi
		if [ -f target/release/examples/pq ]; then
			echo "Release:" $(du -h target/release/examples/pq | awk '{print $1}')
		fi
	fi
