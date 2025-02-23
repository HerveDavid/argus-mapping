use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, TypePath};

pub fn impl_identifiable_trait(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Générer l'implémentation pour tous les champs
    let register_impl = generate_register_impl(&ast.data);

    let expanded = quote! {
        impl Identifiable for #name {
            fn id(&self) -> String {
                self.id.clone()
            }

            fn register(&self, world: &mut bevy_ecs::world::World, schedule: &mut bevy_ecs::schedule::Schedule) {
                // Register self first
                {
                    let mut event_writer = world.resource_mut::<bevy_ecs::event::Events<crate::plugins::RegisterEvent<Self>>>();
                    event_writer.send(RegisterEvent {
                        id: self.id(),
                        component: self.clone(),
                    });
                    dbg!("{:?}", self.id());
                }

                // Then recursively register all identifiable fields
                #register_impl

                schedule.run(world);

                dbg!("apply the world");
            }
        }
    };

    expanded.into()
}

pub fn generate_register_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_registers = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        // Check if field type implements Identifiable
                        if is_identifiable_type(field_type) {
                            // Handle Vec<T> where T: Identifiable
                            if let Type::Path(TypePath { path, .. }) = field_type {
                                if path
                                    .segments
                                    .last()
                                    .map(|s| s.ident == "Vec")
                                    .unwrap_or(false)
                                {
                                    return quote! {
                                        for item in &self.#field_name {
                                            item.register(world, schedule);
                                        }
                                    };
                                }
                            }

                            // Handle single Identifiable field
                            quote! {
                                self.#field_name.register(world, schedule);
                            }
                        } else {
                            quote! {}
                        }
                    });

                    quote! {
                        #(#field_registers)*
                    }
                }
                _ => quote! {
                                dbg!("2");
                },
            }
        }
        _ => quote! {

                                dbg!("3")
        },
    }
}

fn is_identifiable_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        let segments = &type_path.path.segments;
        if let Some(last_segment) = segments.last() {
            // Si c'est un Vec, regarder le type à l'intérieur
            if last_segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return is_identifiable_type(inner_type);
                    }
                }
                return false;
            }

            // Vérifier si le type lui-même est identifiable
            let type_name = last_segment.ident.to_string();
            matches!(
                type_name.as_str(),
                "Substation"
                    | "VoltageLevel"
                    | "Generator"
                    | "Load"
                    | "Line"
                    | "Switch"
                    | "ShuntCompensator"
                    | "StaticVarCompensator"
                    | "DanglingLine"
                    | "TieLine"
                    | "HvdcLine"
                    | "HvdcConverterStation"
                    | "BusbarSection"
                    | "TwoWindingsTransformer"
                    | "ThreeWindingsTransformer"
            )
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_is_identifiable_type_basic() {
        let type_substation: Type = parse_quote!(Substation);
        assert!(
            is_identifiable_type(&type_substation),
            "Substation devrait être identifiable"
        );

        let type_string: Type = parse_quote!(String);
        assert!(
            !is_identifiable_type(&type_string),
            "String ne devrait pas être identifiable"
        );
    }

    #[test]
    fn test_is_identifiable_type_vectors() {
        // Test Vec<Substation>
        let type_vec: Type = parse_quote!(Vec<Substation>);

        // Debug pour voir la structure complète du type
        if let Type::Path(type_path) = &type_vec {
            if let Some(last_segment) = type_path.path.segments.last() {
                println!("Last segment: {:?}", last_segment.ident);
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    println!("Generic args: {:?}", args);
                }
            }
        }

        assert!(
            is_identifiable_type(&type_vec),
            "Vec<Substation> devrait être identifiable"
        );
    }

    #[test]
    fn test_is_identifiable_type_all_types() {
        // Test individuellement chaque type
        let test_cases = [
            (parse_quote!(Substation), true),
            (parse_quote!(VoltageLevel), true),
            (parse_quote!(Generator), true),
            (parse_quote!(Load), true),
            (parse_quote!(Line), true),
            (parse_quote!(Switch), true),
            (parse_quote!(ShuntCompensator), true),
            (parse_quote!(StaticVarCompensator), true),
            (parse_quote!(DanglingLine), true),
            (parse_quote!(TieLine), true),
            (parse_quote!(HvdcLine), true),
            (parse_quote!(HvdcConverterStation), true),
            (parse_quote!(BusbarSection), true),
            (parse_quote!(TwoWindingsTransformer), true),
            (parse_quote!(ThreeWindingsTransformer), true),
            (parse_quote!(String), false),
            (parse_quote!(i32), false),
        ];

        for (type_value, should_be_identifiable) in test_cases.iter() {
            assert_eq!(
                is_identifiable_type(type_value),
                *should_be_identifiable,
                "Type {:?} devrait {}être identifiable",
                type_value,
                if *should_be_identifiable {
                    ""
                } else {
                    "ne pas "
                }
            );
        }
    }

    #[test]
    fn test_type_path_structure() {
        let type_hvdc: Type = parse_quote!(HvdcConverterStation);
        if let Type::Path(type_path) = &type_hvdc {
            let segment = type_path.path.segments.last().unwrap();
            println!("Segment ident: {}", segment.ident);
            println!("Segment span: {:?}", segment.ident.span());

            // Afficher plus de détails sur le chemin
            println!("Path segments:");
            for seg in type_path.path.segments.iter() {
                println!("  - {}", seg.ident);
            }
        }
    }
}
