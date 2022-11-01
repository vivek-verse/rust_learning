extern crate proc_macro;
use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, Ident, ItemFn, Pat, Stmt};

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        let fn_args = extract_arg_names(input);
        let statements = generate_verbose_log(fn_name, fn_args);

        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(
            0,
            parse_quote! {
                println!("[Info] calling {}", stringify!(#fn_name));
            },
        );
    }

    input.to_token_stream().into()
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat) = &(*pat_type.pat) {
                    return Some(&pat.ident);
                }
            }
            None
        })
        .collect()
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt> {
    let mut statements = vec![parse_quote! {
        print!("[Info] calling {} | ", stringify!(#fn_name));
    }];

    for arg in fn_args {
        statements.push(parse_quote! {
            print!("{} = {:?} ", stringify!(#arg), #arg);
        });
    }

    statements.push(parse_quote! { println!(); });

    statements
}
