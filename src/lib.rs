//! `embed-c` is a crate that allows you to embed C code inside Rust code files. The C code is
//! translated into Rust code at compile time using [C2Rust](https://github.com/immunant/c2rust),
//! which means that it is fully interoperable with Rust. C code can call Rust code, and vice-versa.
//!
//! ### Install
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! embed-c = "0.1"
//! ```
//!
//! ### Basic usage
//! ```rust
//! use embed_c::embed_c;
//!
//! embed_c! {
//!     int add(int x, int y) {
//!         return x + y;
//!     }
//! }
//!
//! fn main() {
//!     let x = unsafe { add(1, 2) };
//!     println!("{}", x);
//! }
//! ```
//!
//! ### Limitations
//! Many
//!
//! ### Motivation
//! N/A
//!
//! ### License
//!
//! This project is licensed under either of
//!
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!    https://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!    https://opensource.org/licenses/MIT)
//!
//! at your option.

pub use embed_c_macros::embed_c;

#[cfg(test)]
mod tests
{
    extern crate libc;

    use embed_c_macros::embed_c;

    embed_c! {
        int add(int x, int y)
        {
            return x + y;
        }

        void swap(int* a, int* b)
        {
            int t = *a;
            *a = *b;
            *b = t;
        }

        int partition (int arr[], int low, int high)
        {
            int pivot = arr[high];
            int i = low - 1;

            for (int j = low; j <= high - 1; j++)
            {
                if (arr[j] <= pivot)
                {
                    i++;
                    swap(&arr[i], &arr[j]);
                }
            }
            swap(&arr[i + 1], &arr[high]);
            return i + 1;
        }

        // Quick sort implementation
        void quickSort(int arr[], int low, int high)
        {
            if (low < high)
            {
                int i = partition(arr, low, high);
                quickSort(arr, low, i - 1);
                quickSort(arr, i + 1, high);
            }
        }

        // Duff's device
        // Safely copies an array into another
        void send(to, from, count)
            register short *to, *from;
            register count;
        {
            register n = (count + 7) / 8;
            switch (count % 8) {
            case 0: do { *to++ = *from++;
            case 7:      *to++ = *from++;
            case 6:      *to++ = *from++;
            case 5:      *to++ = *from++;
            case 4:      *to++ = *from++;
            case 3:      *to++ = *from++;
            case 2:      *to++ = *from++;
            case 1:      *to++ = *from++;
                    } while (--n > 0);
            }
        }
    }

    #[test]
    fn test_c_macro()
    {
        unsafe { assert_eq!(add(1, 2), 3); }
    }

    #[test]
    fn test_quicksort()
    {
        let mut arr = [1, 5, 3, 2, 4];
        unsafe { quickSort(arr.as_mut_ptr(), 0, 4); };
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duff()
    {
        let mut source = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut dest = [0; 10];
        unsafe { send(dest.as_mut_ptr(), source.as_mut_ptr(), 10); };
        assert_eq!(source, dest);
    }
}