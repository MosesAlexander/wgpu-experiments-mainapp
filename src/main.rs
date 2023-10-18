use experiments::run;

fn main() {
    pollster::block_on(run());
}
