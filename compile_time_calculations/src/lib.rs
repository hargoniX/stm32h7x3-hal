extern crate proc_macro;
use proc_macro::TokenStream;
use syn;
use quote::quote;

#[proc_macro]
pub fn calc_sys_ck_config(sys_ck: TokenStream) -> TokenStream{
    let sys_ck: syn::LitInt = syn::parse(sys_ck).unwrap();
    let sys_ck = sys_ck.value() as f64;

    let mut best_config: (u32, u32, u32) = (1,4,2);
    let mut closest: f64 = 64_000_000.0;

    for divm in 1..64 {
        for divn in 3..513 {
            for divp in (2..129).step_by(2) {
                let ref_ck: f64 = 64_000_000.0/(divm as f64);
                if ref_ck > 1_000_000.0 && ref_ck < 16_000_000.0 {
                    let pre_divider = ref_ck * (divn as f64);
                    if ref_ck < 2_000_000.0 {
                        if !(pre_divider > 150_000_000.0 && pre_divider < 420_000_000.0) {
                            break;
                        }
                    }
                    else {
                        if !(pre_divider > 192_000_000.0 && pre_divider < 836_000_000.0) {
                            break;
                        }
                    }

                    let pll_p_ck: f64 = pre_divider / (divp as f64);
                    if pll_p_ck < 400_000_000.0 {
                        let difference: f64 = (sys_ck - pll_p_ck).abs();
                        if difference < closest {
                            closest = difference;
                            best_config = (divm, divn, divp);
                        }
                    }
                }
            }
        }
    }

    let divm = best_config.0;
    let divn = best_config.1;
    let divp = best_config.2;
    let config = quote! {
        (#divm, #divn, #divp)
    };
    TokenStream::from(config)
}