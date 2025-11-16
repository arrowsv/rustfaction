use anyhow::{Result};

/// A macro for creating function hooks using MinHook.
/// 
/// Generates:
/// - `fn_name()`: The detour function with the specified name and body.
/// - `fn_name_original()`: A function to call the original using the given address.
/// - `fn_name_register()`: A function to enable the hook.
/// 
/// # Parameters
/// - `addr`: The address of the target function to hook.
/// - `extern "..." fn(...) -> ...`: The function signature.
/// - `name`: The name of the detour function.
/// - `body`: The body of the detour function.
/// 
/// # Example
/// ```ignore
/// create_hook!(
///     addr!(0x123456),
///     extern "C" fn(x: i32) -> i32,
///     my_function
///     {
///         println!("The value of x is {}.", x);
///         my_function_original()(x) // Optionally call original function to resume execution
///     }
/// );
///```
#[macro_export]
macro_rules! hook_fn {
    (
        $addr:expr,
        extern $abi:tt fn($($arg:ident: $ty:ty),*) $(-> $ret:ty)?,
        $name:ident
        $body:block
    ) => {
        paste::paste! {
            // Original function type
            type [<$name:camel T>] = extern $abi fn($($ty),*) $(-> $ret)?;
            
            // Detour function
            #[allow(unused_variables)]
            extern $abi fn $name($($arg: $ty),*) $(-> $ret)? {
                $body
            }
            
            // Original function pointer
            static [<$name:upper _ORIGINAL>]: std::sync::atomic::AtomicPtr<std::ffi::c_void> =
                std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

            /// Creates and enables the hook. Stores the original function pointer for later use
            pub fn [<$name _register>]() -> anyhow::Result<()> {
                use minhook_sys::{MH_CreateHook, MH_EnableHook, MH_OK};
                use std::sync::atomic::Ordering;

                let target = $addr;
                let detour = $name as *mut std::ffi::c_void;
                let original = &[<$name:upper _ORIGINAL>];

                let mut original_ptr = std::ptr::null_mut();
                if unsafe { MH_CreateHook(target as *mut std::ffi::c_void, detour, &mut original_ptr) } != MH_OK {
                    anyhow::bail!("Create hook failed for {:?}", target);
                }

                original.store(original_ptr, Ordering::SeqCst);

                if unsafe { MH_EnableHook(target as *mut std::ffi::c_void) } != MH_OK {
                    anyhow::bail!("Enable hook failed for {:?}", target);
                }

                Ok(())
            }

            /// Retrieves the original function pointer and casts it to the correct type
            #[allow(dead_code)]
            pub fn [<$name _original>]() -> [<$name:camel T>] {
                let ptr = [<$name:upper _ORIGINAL>].load(std::sync::atomic::Ordering::SeqCst);
                unsafe { std::mem::transmute(ptr) }
            }
        }
    };
}

pub fn init_minhook() -> Result<()> {
    if unsafe { minhook_sys::MH_Initialize() } != 0 {
        anyhow::bail!("Failed to initialize MinHook")
    }
    Ok(())
}