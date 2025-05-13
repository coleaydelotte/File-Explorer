mod main_loop;
mod functionality;
mod directory;
mod os_calls;

/**
 * This is the entry point for the application purely for the command line interface.
 */
fn main() {
    main_loop::main_loop();
}