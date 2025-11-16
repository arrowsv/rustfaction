use std::{sync::{atomic::{AtomicUsize, Ordering}}};

pub static MODULE_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_module_base(base: usize) {
    MODULE_BASE.store(base, Ordering::Relaxed);
}

pub fn get_module_base() -> usize {
    MODULE_BASE.load(Ordering::Relaxed)
}

/// Calculates and returns the relative virtual address (RVA) from an absolute address.
/// 
/// # Example
/// ```
/// let address: usize = addr(0x123456); // Returns 0x123456 + module base
/// ```
pub fn addr(address: usize) -> usize {
    get_module_base() + address
}

/// Creates a function that returns a mutable variable pointer to an address.
/// 
/// # Parameters
/// - `addr`: The address.
/// - `t`: The type of the pointer.
/// - `name`: The name of the variable.
/// 
/// # Examples
/// ```ignore
/// addr_var!(addr(0x123456), i32, my_integer);
/// let ptr: *mut i32 = my_integer();
/// unsafe { *ptr = 42; }
/// 
/// addr_var!(addr(0x123456), i32, my_second_integer);
/// unsafe { *my_second_integer() = 100; }
/// ```
#[macro_export]
macro_rules! addr_var {
    (
        $addr:expr, 
        $t:ty, 
        $name:ident
    ) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            static [<$name _address>]: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| {
                $addr
            });
    
            #[allow(dead_code)]
            #[inline(always)]
            pub fn $name() -> *mut $t {
                *[<$name _address>] as *mut $t
            }
        }
    };
}

/// Creates a function that returns a function pointer to an address.
/// 
/// # Parameters
/// - `addr`: The address.
/// - `t`: The type of the pointer.
/// - `name`: The name of the function.
/// 
/// # Example
/// ```ignore
/// addr_fn!(
///     addr(0x123456), 
///     extern "C" fn(x: i32, y: f32), 
///     my_function
/// );
/// my_function()(10, 3.14);
/// ```
#[macro_export]
macro_rules! addr_fn {
    (
        $addr:expr, 
        extern $abi:tt fn($($arg:ident: $ty:ty),*) -> $ret:ty,
        $name:ident
    ) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            static [<$name _address>]: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| {
                $addr
            });
    
            #[allow(dead_code)]
            #[inline(always)]
            pub fn $name() -> extern $abi fn($($arg: $ty),*) -> $ret {
                unsafe { std::mem::transmute(*[<$name _address>] as usize) }
            }
        }
    };
}