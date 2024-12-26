use ixc_schema_macros::SchemaValue;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
pub struct MessageDescriptor<'a> {
    pub request_type: &'a str,
    pub response_type: Option<&'a str>,
    pub events: &'a [&'a str],
    pub error_codes: &'a [ErrorCodeDescriptor<'a>],
}

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, SchemaValue, Default)]
pub struct ErrorCodeDescriptor<'a> {
    pub name: &'a str,
    pub discriminant: u8,
}