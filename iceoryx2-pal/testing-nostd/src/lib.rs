#![cfg_attr(not(feature = "std"), no_std)]

pub use inventory;

pub struct TestCase {
    pub name: &'static str,
    pub test_fn: fn(),
}

inventory::collect!(TestCase);

pub mod internal {
    pub use iceoryx2_pal_print::cout;
}

#[macro_export]
macro_rules! test_suite {
    ($($module_path:literal as $module:ident),* $(,)?) => {
        $(
            #[cfg(not(feature = "std"))]
            #[path = $module_path]
            mod $module;
        )*

        #[cfg(feature = "std")]
        fn main() {}

        #[cfg(not(feature = "std"))]
        #[no_mangle]
        pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
            run_tests()
        }

        #[cfg(not(feature = "std"))]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[cfg(not(feature = "std"))]
        #[no_mangle]
        extern "C" fn rust_eh_personality() {}

        #[cfg(not(feature = "std"))]
        fn run_tests() -> isize {
            // Collect all registered tests
            let tests = $crate::inventory::iter::<$crate::TestCase>();

            let mut passed = 0;
            let mut failed = 0;
            let mut total = 0;

            $crate::internal::cout!("
running tests

");

            for test in tests {
                total += 1;
                $crate::internal::cout!("test ");
                $crate::internal::cout!("{}", test.name);
                $crate::internal::cout!(" ... ");

                (test.test_fn)(); // no panic handling

                $crate::internal::cout!("ok\n");
                passed += 1;
            }

            $crate::internal::cout!(
                "
test result: ",
            );
            if failed == 0 {
                $crate::internal::cout!("ok. ");
            } else {
                $crate::internal::cout!("FAILED. ");
            }

            $crate::internal::cout!("{}", passed);
            $crate::internal::cout!(" passed; ");
            $crate::internal::cout!("{}", failed);
            $crate::internal::cout!(
                " failed

",
            );

            if failed > 0 {
                1
            } else {
                0
            }
        }
    };
}
