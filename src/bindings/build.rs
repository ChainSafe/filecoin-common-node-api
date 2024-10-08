//! This crate interprets all '*.json' files in `/specs` as OpenRPC documents,
//! and generates corresponding modules based on the filename.

use anyhow::{bail, Context as _};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::schema::{
    ArrayValidation, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};
use syn::Ident;

use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/specs/");
    println!("cargo::rerun-if-changed={}", dir);

    let mut modules = TokenStream::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.metadata()?.is_file()
            && path.extension().is_some_and(|it| it == "json")
            && path
                .file_name()
                .is_some_and(|it| !it.to_string_lossy().starts_with('.'))
        {
            modules.extend(
                file2module(&path).context(format!("couldn't process file {}", path.display()))?,
            )
        }
    }

    let pretty = prettyplease::unparse(&syn::parse2(modules).context("internal codegen error")?);
    let mut out = env::var_os("OUT_DIR").context("invalid build script environment")?;
    out.push("/bindings.rs");
    fs::write(out, pretty).context("couldn't write generated code")?;

    Ok(())
}

/// Generate a `pub mod $ident { trait Api { .. } .. }` for a file at the given path.
fn file2module(path: &Path) -> anyhow::Result<TokenStream> {
    let file_stem = path
        .file_stem()
        .and_then(OsStr::to_str)
        .context("bad or no file name")?;
    let mod_name = syn::parse_str::<syn::Ident>(
        file_stem
            .split_once(char::is_whitespace)
            .map(|(before, _after)| before)
            .unwrap_or(file_stem),
    )
    .context("filename must of the form `IDENT` or `IDENT + WHITESPACE + IGNORED`, e.g `v0 some-ignored-comment.json`")?;
    let doc = openrpc_types::resolve_within(
        serde_json::from_reader(File::open(path).context("couldn't open file")?)
            .context("couldn't parse file as OpenRPC document")?,
    )
    .context("couldn't resolve OpenRPC references")?;
    let code = doc2code(doc).context("couldn't generate code")?;
    Ok(quote! {
        pub mod #mod_name {
            #code
        }
    })
}

/// Generates a `trait Api { .. }`, and associated types
fn doc2code(mut doc: openrpc_types::resolved::OpenRPC) -> anyhow::Result<TokenStream> {
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
        pub trait Api {
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

/// OpenRPC uses `#/components/schemas/Foo`,
/// but JSON Schema (and [typify]) uses `#/definitions/Foo`.
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
                // descend into the "array" member of the JSON Schema.
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
                // descend into the "object" member of the JSON Schema.
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
                // descend into the "anyOf"/"oneOf" etc. members of the JSON Schema.
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

/// Recursive functions which accept a trait can end up in a trait solver cycle
/// for e.g `&mut &mut ... FnMut`.
///
/// Break that cycle by going with dynamic dispatch.
fn cast<U>(f: &mut impl FnMut(&mut String) -> U) -> &mut dyn FnMut(&mut String) -> U {
    f as _
}
