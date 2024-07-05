use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
    hash::{BuildHasher, RandomState},
};

use anyhow::{bail, Context as _};
use either::Either;
use ez_jsonrpc_types::{self as jsonrpc, RequestParameters};
use indexmap::IndexMap;
use jsonschema::{CompilationOptions, JSONSchema, ValidationError};
use openrpc_types::{Components, ParamStructure};
use schemars::schema::{Schema, SchemaObject};
use serde::Serialize;
use serde_json::json;

pub struct CheckMethod<S = RandomState> {
    params: IndexMap<String, CheckContentDescriptor, S>,
    param_structure: ParamStructure,
    deprecated: bool,
    result: Option<JSONSchema>,
}

impl CheckMethod {
    pub fn new(
        spec: &openrpc_types::resolved::Method,
        compilation_options: &CompilationOptions,
        components: Option<&Components>,
    ) -> anyhow::Result<Self> {
        Self::new_with_hasher(spec, compilation_options, RandomState::new(), components)
    }
}

impl<S> CheckMethod<S> {
    pub fn new_with_hasher(
        spec: &openrpc_types::resolved::Method,
        compilation_options: &CompilationOptions,
        hasher: S,
        components: Option<&Components>,
    ) -> anyhow::Result<Self>
    where
        S: BuildHasher,
    {
        let param_structure = spec.param_structure.unwrap_or_default();

        let mut params = IndexMap::with_capacity_and_hasher(spec.params.len(), hasher);
        let mut options = false;
        for (ix, param) in spec.params.iter().enumerate() {
            let required = param.required.unwrap_or_default();
            if required
                && matches!(
                    param_structure,
                    ParamStructure::ByPosition | ParamStructure::Either
                )
                && options
            {
                bail!(
                    "parameter at index {} in method {} is out-of-order",
                    ix,
                    spec.name
                )
            }

            if !required {
                options = false
            }

            if params.contains_key(&param.name)
                && matches!(
                    param_structure,
                    ParamStructure::ByName | ParamStructure::Either
                )
            {
                bail!(
                    "parameter `{}` in method {} is duplicated",
                    param.name,
                    spec.name
                )
            }

            params.insert(
                param.name.clone(),
                CheckContentDescriptor {
                    required,
                    deprecated: param.deprecated.unwrap_or_default(),
                    schema: compile(compilation_options, &param.schema, components)?,
                },
            );
        }

        Ok(CheckMethod {
            params,
            param_structure,
            deprecated: spec.deprecated.unwrap_or_default(),
            result: match &spec.result {
                Some(it) => Some(
                    compile(compilation_options, &it.schema, components)
                        .context("couldn't compile JSON Schema")?,
                ),
                None => None,
            },
        })
    }
    pub fn check(
        &self,
        request: &jsonrpc::Request,
        response: Option<&jsonrpc::Response>,
    ) -> Vec<Annotation> {
        let mut annotations = vec![];
        match (self.param_structure, &request.params) {
            (ParamStructure::ByName, Some(RequestParameters::ByPosition(_)))
            | (ParamStructure::ByPosition, Some(RequestParameters::ByName(_))) => {
                annotations.push(Annotation::IncorrectParamStructure);
            }
            _ => {}
        }

        let mut request_params = match &request.params {
            None => Either::Left(VecDeque::new()),
            Some(RequestParameters::ByPosition(it)) => Either::Left(it.iter().collect()),
            Some(RequestParameters::ByName(it)) => Either::Right(
                it.iter()
                    .map(|(k, v)| (Cow::Borrowed(&**k), v))
                    .collect::<HashMap<_, _>>(),
            ),
        };

        for (
            name,
            CheckContentDescriptor {
                required,
                deprecated,
                schema,
            },
        ) in &self.params
        {
            let provided = match &mut request_params {
                Either::Left(by_position) => by_position.pop_front(),
                Either::Right(by_name) => by_name.remove(&**name),
            };
            match (required, provided) {
                (true, None) => annotations.push(Annotation::MissingRequiredParam),
                (_, Some(provided)) => {
                    if *deprecated {
                        annotations.push(Annotation::DeprecatedParam)
                    }
                    if !schema.is_valid(provided) {
                        annotations.push(Annotation::InvalidParam)
                    }
                }
                (false, None) => {}
            }
        }

        if !match request_params {
            Either::Left(it) => it.is_empty(),
            Either::Right(it) => it.is_empty(),
        } {
            annotations.push(Annotation::ExcessParam)
        }

        match (&request.id, &self.result, response) {
            (None, None, None) => {}

            (Some(request_id), Some(schema), Some(jsonrpc::Response { result, id, .. })) => {
                if request_id != id {
                    annotations.push(Annotation::BadNotification)
                }
                if let Ok(result) = result {
                    if !schema.is_valid(result) {
                        annotations.push(Annotation::InvalidResult)
                    }
                }
            }
            _ => annotations.push(Annotation::BadNotification),
        }
        if self.deprecated {
            annotations.push(Annotation::DeprecatedMethod)
        }
        annotations
    }
}

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Annotation {
    IncorrectParamStructure,
    MissingRequiredParam,
    DeprecatedParam,
    InvalidParam,
    InvalidResult,
    ExcessParam,
    BadNotification,
    DeprecatedMethod,
}

struct CheckContentDescriptor {
    required: bool,
    deprecated: bool,
    schema: JSONSchema,
}

fn compile(
    compilation_options: &CompilationOptions,
    schema: &Schema,
    components: Option<&Components>,
) -> Result<JSONSchema, ValidationError<'static>> {
    #[derive(Serialize)]
    struct Bundle<'a> {
        #[serde(flatten)]
        schema: &'a SchemaObject,
        components: Option<&'a openrpc_types::Components>,
    }
    let json = match schema {
        Schema::Bool(it) => json!(it),
        Schema::Object(schema) => serde_json::to_value(Bundle { schema, components }).unwrap(),
    };
    compilation_options.compile(&json).map_err(
        |ValidationError {
             instance,
             kind,
             instance_path,
             schema_path,
         }| {
            ValidationError {
                instance: Cow::Owned(instance.into_owned()),
                kind,
                instance_path,
                schema_path,
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_path_for_jsonschema() {
        let checker = JSONSchema::compile(&json!({
            "$ref": "#/components/schemas/foo",
            "components": {
                "schemas": {
                    "foo": {
                        "type": "string"
                    }
                }
            }
        }))
        .unwrap();
        assert!(checker.is_valid(&json!("my_string")));
    }
}
