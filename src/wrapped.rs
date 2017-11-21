use core::{slice, ops};
use ext;

#[repr(C)]
struct Descriptor {
	args_ptr: *const u8,
	args_len: usize,
	result_ptr: *const u8,
	result_len: usize,
}

/// Input data of a contract.
///
/// Basically it can be viewed as
/// a byte slice (`&[u8]`) and it has `Deref<Target=[u8]>` impl indeed.
///
/// You should use [`parse_args`] to acquire `WrappedArgs`.
///
/// # Examples
///
/// ```rust
/// use pwasm_std::WrappedArgs;
///
/// fn takes_slice(input: &[u8]) {
/// 	// ...
/// 	# input.len(); // to silence unused var warnings
/// }
///
/// #[no_mangle]
/// pub fn call(descriptor: *mut u8) {
/// 	let (input, result): (WrappedArgs, _) = unsafe { pwasm_std::parse_args(descriptor) };
/// 	takes_slice(&input);
/// }
/// ```
///
/// [`parse_args`]: fn.parse_args.html
pub struct WrappedArgs {
	desc: *const Descriptor
}

impl ops::Deref for WrappedArgs {
	type Target = [u8];
    fn deref(&self) -> &Self::Target {
		unsafe {
			let ptr = (*self.desc).args_ptr;
			let len = (*self.desc).args_len;
			if len == 0 {
				// It is UB to create a slice with null ptr.
				&[]
			} else {
				slice::from_raw_parts(ptr, len)
			}
		}
	}
}

impl AsRef<[u8]> for WrappedArgs {
	fn as_ref(&self) -> &[u8] {
		&*self
	}
}

/// Writeable handle of execution results.
///
/// You can use this handle to write execution results of your contract.
pub struct WrappedResult;

impl WrappedResult {
	/// Finalize writing result into the descriptor
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// # use pwasm_std::parse_args;
	/// # let result = unsafe { parse_args(::std::ptr::null_mut()).1 };
	/// let data: Vec<u8> = vec![0, 1, 2, 3];
	/// result.done(data);
	/// ```
	///
	/// You can also return static data.
	///
	/// ```rust,no_run
	/// # use pwasm_std::parse_args;
	/// # let result = unsafe { parse_args(::std::ptr::null_mut()).1 };
	/// let data: &'static [u8] = &[0, 1, 2, 3];
	/// result.done(data);
	/// ```
	pub fn done<T: AsRef<[u8]>>(self, val: T) -> ! {
		let result = val.as_ref();
		ext::return_(result);
		// Control flow can't get here so `val` doesn't get dropped.
	}
}

/// Parse decriptor into wrapped args and result.
///
/// # Safety
///
/// `ptr` should be non-null and point to a valid descriptor.
///
/// # Examples
///
/// ```rust
/// #[no_mangle]
/// pub fn call(descriptor: *mut u8) {
/// 	let (input, result) = unsafe { pwasm_std::parse_args(descriptor) };
/// 	let echo: Vec<u8> = input.to_vec();
/// 	result.done(echo);
/// }
/// ```
///
pub unsafe fn parse_args(ptr: *mut u8) -> (WrappedArgs, WrappedResult) {
	let desc = ptr as *mut Descriptor;
	let args = WrappedArgs { desc: desc };
	let result = WrappedResult;
	(args, result)
}
