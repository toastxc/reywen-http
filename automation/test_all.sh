# compiling
mold --run cargo b --features serde --features hyper_engine
mold --run cargo b --features serde --features reqwasm_engine
mold --run cargo b --features serde --features reqwest_engine

# testing
mold --run cargo test --lib hyper --features serde --features hyper_engine
mold --run cargo test --lib reqwest --features serde --features reqwest_engine
#todo! WASM
