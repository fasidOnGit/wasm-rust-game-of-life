#![feature(test)]
extern crate test;
extern crate wasm_rust_game_of_life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_rust_game_of_life::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
