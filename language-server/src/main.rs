mod completion;
mod diagnostics;
mod document;
mod goto;
mod handler;
mod hover;
mod references;
mod rename;
mod server;
mod signature;
mod symbols;

fn main() {
    server::run();
}
