extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote};
use syn;

struct PinDesc {
    name: Ident,
    kind: Ident,
    pins: Vec<syn::LitInt>
}

enum Member {
    Managed(PinDesc),
    Unmanaged(syn::Field)
}

fn parse_member(field: &syn::Field) -> Member {
    if let Some(name) = &field.ident {
        if let syn::Type::Path(path) = &field.ty {
            if let Some(last_item) = path.path.segments.last() {
                let mut pins = Vec::new();
                if let syn::PathArguments::AngleBracketed(args) = &last_item.arguments {
                    for pin_id in args.args.iter() {
                        if let syn::GenericArgument::Const(syn::Expr::Lit(id)) = pin_id {
                            if let syn::Lit::Int(id_int) = &id.lit {
                                pins.push(id_int.clone());
                            }
                        }
                    }
                }

                match last_item.ident.to_string().as_str() {
                    "In" => {
                        if pins.len() == 1 {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("In", Span::call_site())})
                        } else {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("InBus", Span::call_site())})
                        }
                    },
                    "Out" => {
                        if pins.len() == 1 {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("Out", Span::call_site())})
                        } else {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("OutBus", Span::call_site())})
                        }
                    },
                    "InOut" => {
                        if pins.len() == 1 {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("InOut", Span::call_site())})
                        } else {
                            Member::Managed(PinDesc{name: name.clone(), pins, kind: Ident::new("InOutBus", Span::call_site())})
                        }
                    },
                    _ => Member::Unmanaged(field.clone())
                }
            } else {
                Member::Unmanaged(field.clone())
            }
        } else {
            Member::Unmanaged(field.clone())
        }
    } else {
        Member::Unmanaged(field.clone())
    }
}

fn make_comp(ast: syn::ItemStruct) -> TokenStream {
    let struct_name = ast.ident;
    let mut descs : Vec<PinDesc> = Vec::new();
    let mut members : Vec<syn::Field> = Vec::new();

    if let syn::Fields::Named(nameds) = ast.fields {
        for field in nameds.named.iter() {
            match parse_member(field) {
                Member::Managed(pin_desc) => {
                    descs.push(pin_desc);
                },
                Member::Unmanaged(member) => {
                    members.push(member);
                }
            }
        }
    }

    let mut pins = Vec::new();
    let mut news = Vec::new();
    let mut connects = Vec::new();
    let mut pins_desc = Vec::new();
    let mut pins_create = Vec::new();
    let mut total_pin_count: usize = 0;
    let mut get_names = Vec::new();

    for desc in descs {
        let name = desc.name;
        let kind = desc.kind;
        pins.push(quote!(#name: #kind));
        total_pin_count += desc.pins.len();
        if desc.pins.len() == 1 {
            let pin = &desc.pins[0];
            news.push(quote!(
                #name: #kind::new(#pin)
            ));
            let str_name = format!("{}", name);
            get_names.push(quote!(
                #pin => #str_name
            ));
        } else {
            let pins = &desc.pins;
            news.push(quote!(
                #name: #kind::new(&[#(#pins,)*])
            ));
            for (i, pin) in pins.iter().enumerate() {
                let str_name = format!("{}{}", name, i);
                get_names.push(quote!(
                    #pin => #str_name
                ));
            }
        }
        connects.push(quote!(
            self.#name.connect(bus.clone())
        ));
        let pins_count = desc.pins.len();
        pins_desc.push(quote!(
            pub #name: [usize; #pins_count]
        ));
        let pins_id = desc.pins;
        pins_create.push(quote!(
            #name: [#(#pins_id),*]
        ));
    }

    for unmanaged in &members {
        let name = &unmanaged.ident;
        let type_name = &unmanaged.ty;
        news.push(quote!(
            #name: #type_name::new()
        ));
    };

    let name_pin = quote::format_ident!("{}Pin", struct_name);

    let gen = quote! {
        pub struct #struct_name {
            #(#members,)*
            #(#pins,)*
        }

        pub struct #name_pin {
            #(#pins_desc,)*
        }

        impl #name_pin {
            pub fn len(&self) -> usize {
                #total_pin_count
            }
        }

        impl caemu::component::Connect for #struct_name {
            fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
                #(#connects;)*
            }
            fn get_name(&self, id: usize) -> String {
                String::from(match id {
                    #(#get_names,)*
                    _ => panic!("Unknown pin id {}", id)
                })
            }
        }

        impl #struct_name {
            pub fn new() -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Self {
                    #(#news,)*
                }))
            }

            pub fn get_pins(&self) -> #name_pin {
                #name_pin {
                    #(#pins_create,)*
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn comp(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(item).unwrap();

    make_comp(ast)
}
