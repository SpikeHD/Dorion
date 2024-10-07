fn main() {
  patch_crate::run().expect("Failed while patching");
  tauri_build::build()
}
