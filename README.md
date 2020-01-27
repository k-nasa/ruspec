# ruspec - write like Rspec testing framework with rust

[![crate-name at crates.io](https://img.shields.io/crates/v/ruspec.svg)](https://crates.io/crates/ruspec)

## how to use 

Add your Cargo.toml
```
ruspec = "0.1.3"
```

import ruspec!
```
use ruspec::ruspec;
```

## Example syntax 
```rust
use ruspec::ruspec;

ruspec! {
    describe "test module name" {
        before { let context = 5; }
        subject { context + 5 }

        it "test name" {
            assert_eq!(subject, 10);
        }
    }

    describe "test module 2" {
        before { let context = 5; }
        it "test name" {
            assert_eq!(context, 5);
        }

        context "context is 6" {
            before { let context = 6; }
            it "should equal 6" {
                assert_eq!(context, 6);
            }
        }
    }
}

// # Expand
mod test_module_name {
    #[test]
    fn test_name() {
        let context = 5;

        assert_eq(context + 5, 10)
    }
}

mod test_module_2 {
    #[test]
    fn test_name() {
        let context = 5;

        assert_eq(context, 10)
    }

    mod context_is_6 {
        #[test]
        fn should_equal_6() {
            let context = 6;
            assert_eq!(context, 6)
        }
    }
}
```
