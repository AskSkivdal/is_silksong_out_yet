# Is silksong out yet?
Thats the question we are all having. So i made a crate to check. This crate does not require any 
internet and is highly performant.

### Useage
To add the dependency run
```
cargo add is-silksong-out-yet
```
or add
```
is-silksong-out-yet = "*"
```
to Cargo.toml

```rs
use is_silksong_out_yet::is_silksong_out_yet;

fn main() {
    if is_silksong_out_yet() {
        println!("YAY!")
    } else {
        println!("RELEASE ALREADY, I WANT TO PLAY")
    }
}
```