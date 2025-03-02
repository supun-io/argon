extern crate lua;

fn main() {
    let mut state = lua::State::new();
    state.open_libs();
    let _ = state.do_string("print('hello world!')");
}
