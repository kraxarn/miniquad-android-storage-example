#[cfg(target_os = "linux")]
fn main() {
	println!("Hello, world!");
}

const VALUE: &str = "Hello, Android!";

#[cfg(target_os = "android")]
fn main() {
	let path_string = unsafe {
		let internal_data_path = sapp_android::sapp_internal_data_path();
		let c_str = std::ffi::CStr::from_ptr(internal_data_path);
		c_str.to_string_lossy().into_owned()
	};

	let path = std::path::PathBuf::from(path_string);
	std::fs::create_dir_all(&path);
	let file_path = path.join("file.txt");
	std::fs::write(file_path, VALUE);

	assert_eq!(std::fs::read_to_string(file_path), VALUE);
}
