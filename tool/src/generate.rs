use anyhow::{bail, Context as _};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::schema::{
    ArrayValidation, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};
use syn::Ident;

pub fn generate(
    mut doc: openrpc_types::resolved::OpenRPC,
    trait_name: Ident,
) -> anyhow::Result<TokenStream> {
    if let Some(components_schemas) = doc.components.as_mut().and_then(|it| it.schemas.as_mut()) {
        for schema in components_schemas.values_mut() {
            rewrite_references(schema, |it| {
                let name = extract(it)?;
                *it = format!("#/definitions/{}", name);
                Ok(())
            })?
        }
    }

    let mut space = typify::TypeSpace::default();
    space.add_root_schema(RootSchema {
        meta_schema: None,
        schema: SchemaObject::default(),
        definitions: doc
            .components
            .unwrap_or_default()
            .schemas
            .unwrap_or_default(),
    })?;

    let method_ir = doc
        .methods
        .into_iter()
        .map(|method| {
            anyhow::Ok(Method {
                name: method.name,
                result: method
                    .result
                    .as_ref()
                    .map(|cd| space.add_type(&cd.schema))
                    .transpose()?,
                params: method
                    .params
                    .into_iter()
                    .map(|cd| space.add_type(&cd.schema).map(|id| (cd.name, id)))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let method_code = method_ir.iter().try_fold(
        TokenStream::new(),
        |mut acc,
         Method {
             name,
             result,
             params,
         }| {
            let fn_name = syn::parse_str::<Ident>(&name.replace('.', "_"))?;
            let ret = result
                .as_ref()
                .map(|id| space.get_type(id).map(|ty| ty.ident()))
                .transpose()?;
            let vars = params
                .iter()
                .map(|(name, _)| syn::parse_str::<Ident>(name))
                .collect::<Result<Vec<_>, _>>()?;
            let params = params
                .iter()
                .map(|(name, id)| {
                    let name = syn::parse_str::<Ident>(name)?;
                    let ty = space.get_type(id)?.parameter_ident();
                    anyhow::Ok(quote! { #name: #ty })
                })
                .collect::<Result<Vec<_>, _>>()?;
            match ret {
                Some(ret) => acc.extend(quote! {
                    fn #fn_name(&mut self, #(#params),*) -> Result<#ret, Self::Error> {
                        self.call(#name, (#(#vars,)*))
                    }
                }),
                None => bail!("notifications are not currently supported"),
            };
            anyhow::Ok(acc)
        },
    )?;

    Ok(quote! {
        #![allow(clippy::to_string_trait_impl, clippy::clone_on_copy)]
        use serde::{Serialize, Deserialize, de::DeserializeOwned};

        #[allow(non_snake_case, unused)]
        pub trait #trait_name {
            type Error;
            fn call<T: DeserializeOwned>(
                &mut self,
                method: impl Into<String>,
                params: impl ez_jsonrpc::params::SerializePositional
            ) -> Result<T, Self::Error>;
            #method_code
        }

        #space
    })
}

struct Method {
    name: String,
    result: Option<typify::TypeId>,
    params: Vec<(String, typify::TypeId)>,
}

fn extract(s: &str) -> anyhow::Result<&str> {
    let (_whole, grp) = lazy_regex::regex_captures!("^#/components/schemas/(.*)", s)
        .context(format!("unsupported reference format in `{}`", s))?;
    Ok(grp)
}

fn rewrite_references(
    schema: &mut Schema,
    mut rewrite: impl FnMut(&mut String) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if let Schema::Object(SchemaObject {
        reference,
        subschemas,
        array,
        object,
        ..
    }) = schema
    {
        reference.as_mut().map(cast(&mut rewrite)).transpose()?;

        for child in array
            .as_deref_mut()
            .iter_mut()
            .flat_map(
                |ArrayValidation {
                     items,
                     additional_items,
                     contains,
                     ..
                 }| {
                    [additional_items, contains]
                        .into_iter()
                        .flat_map(Option::as_deref_mut)
                        .chain(items.iter_mut().flat_map(|it| match it {
                            SingleOrVec::Vec(it) => &mut it[..],
                            SingleOrVec::Single(it) => std::slice::from_mut(&mut **it),
                        }))
                },
            )
            .chain(object.as_deref_mut().iter_mut().flat_map(
                |ObjectValidation {
                     properties,
                     pattern_properties,
                     additional_properties,
                     property_names,
                     ..
                 }| {
                    [additional_properties, property_names]
                        .into_iter()
                        .flat_map(Option::as_deref_mut)
                        .chain(properties.values_mut())
                        .chain(pattern_properties.values_mut())
                },
            ))
            .chain(subschemas.as_deref_mut().into_iter().flat_map(
                |SubschemaValidation {
                     all_of,
                     any_of,
                     one_of,
                     not,
                     if_schema,
                     then_schema,
                     else_schema,
                 }| {
                    [not, if_schema, then_schema, else_schema]
                        .into_iter()
                        .flat_map(Option::as_deref_mut)
                        .chain([all_of, any_of, one_of].into_iter().flatten().flatten())
                },
            ))
        {
            rewrite_references(child, cast(&mut rewrite))?;
        }
    }
    Ok(())
}

// break trait solver cycles
fn cast<U>(f: &mut impl FnMut(&mut String) -> U) -> &mut dyn FnMut(&mut String) -> U {
    f as _
}
