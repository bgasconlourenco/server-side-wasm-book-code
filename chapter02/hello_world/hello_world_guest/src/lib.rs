#[no_mangle] //mangle as a directive to compiler to not mangle the function name
pub fn greet(ptr: i32, len: i32) {
  let hello = "Hello, "; //create prefix string - autiomatically converted to a string slice

  let input_ptr = ptr as *mut u8; // convert the input pointer to a mutable pointer of u8
  let input_len = len as usize; // convert the input length to usize

  let new_len = input_len + hello.len(); //calculate the new length of string

  let output = unsafe { core::slice::from_raw_parts_mut(0 as *mut u8, new_len) }; //acess linear memory using unsafe block - grab a mutable slice of u8

  output[..hello.len()].copy_from_slice(hello.as_bytes()); //copy the hello string into the output slice
  output[hello.len()..] 
    .copy_from_slice(unsafe { core::slice::from_raw_parts(input_ptr, input_len) }); //copy the input string into the output slice


}
