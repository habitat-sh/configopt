use crate::configopt_type::parse::ParsedField;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn for_struct(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let self_field = quote! {self.#field_ident};
        let span = field.span();
        let serde_name = field.serde_name();
        if field.is_subcommand() {
            quote! {}
        } else if field.is_serde_flatten() {
            quote_spanned! {span=>
                result = format!("{}{}", result, #self_field.toml_config_with_prefix(&serde_prefix));
            }
        }  else {
            let structopt_name = field.structopt_name();
            let structopt_rename = field.structopt_rename();
            let generate_config_arg_name = structopt_rename.rename("generate-config"); 
            let config_files_arg_name = structopt_rename.rename("config-files"); 
            quote_spanned! {span=>
                let key = if serde_prefix.is_empty() {
                    String::from(#serde_name)
                } else {
                    format!("{}.{}", serde_prefix.join("."), #serde_name)
                };
                // Pull out the comment from the clap::App
                let mut comment = String::new();
                let mut hidden = false;
                for arg in &app.p.flags {
                    let b = &arg.b;
                    if #structopt_name == b.name {
                        if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                            hidden = true;
                            break;
                        }
                        comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                        break;
                    }
                }
                if comment.is_empty() && !hidden {
                    for arg in &app.p.opts {
                        let b = &arg.b;
                        if #structopt_name == b.name {
                            if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                                hidden = true;
                                break;
                            }
                            comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                            break;
                        }
                    }
                }
                if comment.is_empty() && !hidden {
                    for (_, arg) in &app.p.positionals {
                        let b = &arg.b;
                        if #structopt_name == b.name {
                            if b.is_set(::structopt::clap::ArgSettings::Hidden) {
                                hidden = true;
                                break;
                            }
                            comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                            break;
                        }
                    }
                }
                if !hidden && !&[#generate_config_arg_name, #config_files_arg_name].contains(&#structopt_name) {
                    if !comment.is_empty() {
                        comment = comment.lines().map(|l| format!("### {}\n", l)).collect::<String>();
                    }
                    match toml::Value::try_from(&#self_field) {
                        Ok(val) => {
                            use toml::value::Value;
                            match &val {
                                Value::Array(a) if a.is_empty() => {
                                    result = format!("{}{}# {} = {}\n\n", result, comment, key, val);
                                }
                                _ => {
                                    result = format!("{}{}{} = {}\n\n", result, comment, key, val);
                                }
                            }
                        }
                        Err(e) if e.to_string() == "unsupported None value" => {
                            result = format!("{}{}# {} =\n\n", result, comment, key);
                        }
                        _ => {}
                    }
                }
            }
        }
    });
    quote! {
        let mut result = String::new();
        #(#field_tokens)*
        result
    }
}
