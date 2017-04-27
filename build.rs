#[macro_use]
extern crate quote;
extern crate syn;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use syn::{Ident, IntTy, Lit};

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.starts_with("thumbv6m") {
        println!("cargo:rustc-cfg=thumbv6m");
    }

    let bits = if env::var_os("CARGO_FEATURE_P2").is_some() {
        2
    } else if env::var_os("CARGO_FEATURE_P3").is_some() {
        3
    } else if env::var_os("CARGO_FEATURE_P4").is_some() {
        4
    } else if env::var_os("CARGO_FEATURE_P5").is_some() {
        5
    } else {
        panic!(
            "Specify the number of priority bits through one of these Cargo \
                features: P2, P3, P4 or P5"
        );
    };

    let n = Lit::Int(bits, IntTy::Unsuffixed);
    let mut tokens = vec![];
    tokens.push(
        quote! {
            const PRIORITY_BITS: u8 = #n;
        },
    );

    // Ceilings
    for i in 0..(1 << bits) + 1 {
        let c = Ident::new(format!("C{}", i));
        let u = Ident::new(format!("U{}", i));

        let doc = format!("A ceiling of {}", i);
        tokens.push(
            quote! {
                #[doc = #doc]
                pub type #c = C<::typenum::#u>;
            },
        );
    }

    // Priorities
    for i in 0..(1 << bits) + 1 {
        let p = Ident::new(format!("P{}", i));
        let u = Ident::new(format!("U{}", i));

        let doc = if i == 0 {
            format!("A priority of 0, the lowest priority")
        } else {
            format!(
                "A priority of {}{}",
                i,
                if i == (1 << bits) {
                    ", the highest priority"
                } else {
                    ""
                }
            )
        };
        tokens.push(
            quote! {
                #[doc = #doc]
                pub type #p = P<::typenum::#u>;
            },
        );
    }

    // GreaterThanOrEqual & LessThanOrEqual
    for i in 0..(1 << bits) + 1 {
        for j in 0..(i + 1) {
            let i = Ident::new(format!("U{}", i));
            let j = Ident::new(format!("U{}", j));

            tokens.push(
                quote! {
                    unsafe impl GreaterThanOrEqual<::typenum::#j> for
                        ::typenum::#i {}

                    unsafe impl LessThanOrEqual<::typenum::#i> for
                        ::typenum::#j {}
                },
            );
        }
    }

    let u = Ident::new(format!("U{}", (1 << bits)));
    let c = Ident::new(format!("C{}", (1 << bits)));
    tokens.push(
        quote! {
            /// Maximum ceiling
            pub type CMAX = #c;

            /// Maximum priority level
            pub type UMAX = ::typenum::#u;
        },
    );

    let tokens = quote! {
        #(#tokens)*
    };

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut out = File::create(PathBuf::from(out_dir).join("prio.rs")).unwrap();

    out.write_all(tokens.as_str().as_bytes()).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
