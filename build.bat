cargo build --release --target wasm32-unknown-unknown
copy target\wasm32-unknown-unknown\release\checkers_cpu.wasm docs\checkers_cpu.wasm
wasm-strip.exe docs\checkers_cpu.wasm
