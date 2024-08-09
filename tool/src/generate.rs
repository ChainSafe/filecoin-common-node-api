use std::collections::HashSet;

use anyhow::Context as _;
use schemars::schema::{
    ArrayValidation, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};

pub fn process(mut doc: openrpc_types::resolved::OpenRPC) -> anyhow::Result<RootSchema> {
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

    for method in doc.methods {
        let name = method.name;
        let result = method
            .result
            .as_ref()
            .map(|cd| space.add_type(&cd.schema))
            .transpose()?;
        let params = method
            .params
            .iter()
            .map(|cd| space.add_type(&cd.schema).map(|id| (&*cd.name, id)))
            .collect::<Result<Vec<_>, _>>()?;
    }
    todo!()
}

fn extract(s: &str) -> anyhow::Result<&str> {
    let (_whole, grp) = lazy_regex::regex_captures!("^#/components/schemas/(.*)", s)
        .context(format!("unsupported reference format for `{}`", s))?;
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
        reference.as_mut().map(&mut rewrite).transpose()?;

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
            rewrite_references(child, &mut rewrite)?;
        }
    }
    Ok(())
}
