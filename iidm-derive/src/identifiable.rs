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
                }

                // Then recursively register all identifiable fields
                #register_impl

                schedule.run(world);
            }
        }
    };

    expanded.into()
}

fn generate_register_impl(data: &Data) -> TokenStream {
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
                _ => quote! {},
            }
        }
        _ => quote! {},
    }
}

fn is_identifiable_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        let type_name = type_path.path.segments.last().unwrap().ident.to_string();
        // Add here all your types that implement Identifiable
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
        )
    } else {
        false
    }
}
