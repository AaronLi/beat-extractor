extern crate embed_resource;

fn main() {
    embed_resource::compile("windows_icon.rc", embed_resource::NONE);
}