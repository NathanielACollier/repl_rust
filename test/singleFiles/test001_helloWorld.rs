
/*
compile with
- rustc test001_helloWorld.rs

# Install rust on arch
```
# Install rustup
sudo pacman -S rustup
# setup rustup to stable which will download all the rust tools including rustc to compile
rustup default stable
# make sure rustc works
rustc -version
```
*/

fn main(){
    println!("Hello World!");
}