use std::collections::HashSet;

use syn::{Attribute, Expr, ExprUnary, Lit, Path, UnOp};

use crate::{index::IndexValue, label::LabelValue};

/**
The `tag` attribute.

This attribute specifies a path to an `sval::Tag` to use
for the annotated item.
*/
pub(crate) struct TagAttr;

impl SvalAttribute for TagAttr {
    type Result = syn::Path;

    fn try_from_expr(&self, expr: &Expr) -> Option<Self::Result> {
        match expr {
            Expr::Lit(lit) => Some(self.from_lit(&lit.lit)),
            Expr::Path(path) => Some(path.path.clone()),
            _ => None,
        }
    }

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Str(ref s) = lit {
            s.parse().expect("invalid value")
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for TagAttr {
    fn key(&self) -> &str {
        "tag"
    }
}

/**
The `data_tag` attribute.

This attribute specifies a path to an `sval::Tag` to use
for the data of the annotated item.
 */
pub(crate) struct DataTagAttr;

impl SvalAttribute for DataTagAttr {
    type Result = syn::Path;

    fn try_from_expr(&self, expr: &Expr) -> Option<Self::Result> {
        match expr {
            Expr::Lit(lit) => Some(self.from_lit(&lit.lit)),
            Expr::Path(path) => Some(path.path.clone()),
            _ => None,
        }
    }

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Str(ref s) = lit {
            s.parse().expect("invalid value")
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for DataTagAttr {
    fn key(&self) -> &str {
        "data_tag"
    }
}

/**
The `label` attribute.

This attribute specifies an `sval::Label` as a constant
to use for the annotated item.
*/
pub(crate) struct LabelAttr;

impl SvalAttribute for LabelAttr {
    type Result = LabelValue;

    fn try_from_expr(&self, expr: &Expr) -> Option<Self::Result> {
        match expr {
            Expr::Lit(lit) => Some(self.from_lit(&lit.lit)),
            Expr::Path(path) => Some(LabelValue::Ident(quote!(#path))),
            _ => None,
        }
    }

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Str(ref s) = lit {
            LabelValue::Const(s.value())
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for LabelAttr {
    fn key(&self) -> &str {
        "label"
    }
}

/**
The `index` attribute.

This attribute specifies an `sval::Index` as a constant
to use for the annotated item.
*/
pub(crate) struct IndexAttr;

impl IndexAttr {
    fn const_from_lit(&self, lit: &Lit) -> isize {
        if let Lit::Int(ref n) = lit {
            n.base10_parse().expect("invalid value")
        } else {
            panic!("unexpected value")
        }
    }
}

impl SvalAttribute for IndexAttr {
    type Result = IndexValue;

    fn try_from_expr(&self, expr: &Expr) -> Option<Self::Result> {
        match expr {
            // Take `-` into account
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_),
                expr,
                ..
            }) => {
                if let Expr::Lit(ref lit) = **expr {
                    Some(IndexValue::Const(-(self.const_from_lit(&lit.lit))))
                } else {
                    None
                }
            }
            Expr::Lit(lit) => Some(IndexValue::Const(self.const_from_lit(&lit.lit))),
            Expr::Path(path) => Some(IndexValue::Ident(quote!(#path))),
            _ => None,
        }
    }

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        IndexValue::Const(self.const_from_lit(lit))
    }
}

impl RawAttribute for IndexAttr {
    fn key(&self) -> &str {
        "index"
    }
}

/**
The `skip` attribute.

This attribute signals that an item should be skipped
from streaming.
*/
pub(crate) struct SkipAttr;

impl SvalAttribute for SkipAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for SkipAttr {
    fn key(&self) -> &str {
        "skip"
    }
}

/**
The `unlabeled_fields` attribute.

This attribute signals that all fields should be unlabeled.
*/
pub(crate) struct UnlabeledFieldsAttr;

impl SvalAttribute for UnlabeledFieldsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for UnlabeledFieldsAttr {
    fn key(&self) -> &str {
        "unlabeled_fields"
    }
}

/**
The `unindexed_fields` attribute.

This attribute signals that all fields should be unindexed.
*/
pub(crate) struct UnindexedFieldsAttr;

impl SvalAttribute for UnindexedFieldsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for UnindexedFieldsAttr {
    fn key(&self) -> &str {
        "unindexed_fields"
    }
}

/**
The `unlabeled_variants` attribute.

This attribute signals that all variants should be unlabeled.
*/
pub(crate) struct UnlabeledVariantsAttr;

impl SvalAttribute for UnlabeledVariantsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for UnlabeledVariantsAttr {
    fn key(&self) -> &str {
        "unlabeled_variants"
    }
}

/**
The `unindexed_variants` attribute.

This attribute signals that all variants should be unindexed.
*/
pub(crate) struct UnindexedVariantsAttr;

impl SvalAttribute for UnindexedVariantsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for UnindexedVariantsAttr {
    fn key(&self) -> &str {
        "unindexed_variants"
    }
}

/**
The `dynamic` attribute.

This attribute signals that an enum should be dynamic.
*/
pub(crate) struct DynamicAttr;

impl SvalAttribute for DynamicAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for DynamicAttr {
    fn key(&self) -> &str {
        "dynamic"
    }
}

/**
The `transparent` attribute.

This attribute signals that a newtype should stream its inner field
without wrapping it in a tag.
*/
pub(crate) struct TransparentAttr;

impl SvalAttribute for TransparentAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for TransparentAttr {
    fn key(&self) -> &str {
        "transparent"
    }
}

/**
The `flatten` attribute.

This attribute will flatten the fields of a value onto its parent.
 */
pub(crate) struct FlattenAttr;

impl SvalAttribute for FlattenAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        #[cfg(not(feature = "flatten"))]
        {
            let _ = lit;
            panic!("the `flatten` attribute can only be used when the `flatten` Cargo feature of `sval_derive` is enabled");
        }
        #[cfg(feature = "flatten")]
        {
            if let Lit::Bool(ref b) = lit {
                b.value
            } else {
                panic!("unexpected value")
            }
        }
    }
}

impl RawAttribute for FlattenAttr {
    fn key(&self) -> &str {
        "flatten"
    }
}

pub(crate) trait RawAttribute {
    fn key(&self) -> &str;
}

pub(crate) trait SvalAttribute: RawAttribute {
    type Result: 'static;

    fn try_from_expr(&self, expr: &Expr) -> Option<Self::Result> {
        if let Expr::Lit(lit) = expr {
            Some(self.from_lit(&lit.lit))
        } else {
            None
        }
    }

    fn from_lit(&self, lit: &Lit) -> Self::Result;
}

pub(crate) fn ensure_empty(ctxt: &str, attrs: &[Attribute]) {
    // Just ensure the attribute list is empty
    for (value_key, _) in attrs
        .iter()
        .filter_map(|attr| sval_attr(ctxt, attr))
        .flatten()
    {
        panic!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt);
    }
}

pub(crate) fn ensure_missing<T: SvalAttribute>(ctxt: &str, request: T, attrs: &[Attribute]) {
    let key = request.key().to_owned();

    if get_unchecked::<T>(ctxt, request, attrs).is_some() {
        panic!("unsupported attribute `{}` on {}", key, ctxt);
    }
}

pub(crate) fn check(ctxt: &str, allowed: &[&dyn RawAttribute], attrs: &[Attribute]) {
    let mut seen = HashSet::new();

    for (value_key, _) in attrs
        .iter()
        .filter_map(|attr| sval_attr(ctxt, attr))
        .flatten()
    {
        let mut is_valid_attr = false;

        for attr in allowed {
            let attr_key = attr.key();

            if value_key.is_ident(attr_key) {
                is_valid_attr = true;

                if !seen.insert(attr_key) {
                    panic!("duplicate attribute `{}` on {}", quote!(#value_key), ctxt);
                }
            }
        }

        if !is_valid_attr {
            panic!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt);
        }
    }
}

pub(crate) fn get_unchecked<T: SvalAttribute>(
    ctxt: &str,
    request: T,
    attrs: &[Attribute],
) -> Option<T::Result> {
    let request_key = request.key();

    for (value_key, value) in attrs
        .iter()
        .filter_map(|attr| sval_attr(ctxt, attr))
        .flatten()
    {
        if value_key.is_ident(request_key) {
            return Some(request.try_from_expr(&value).expect("unexpected value"));
        }
    }

    None
}

fn sval_attr<'a>(
    ctxt: &'a str,
    attr: &'_ Attribute,
) -> Option<impl IntoIterator<Item = (Path, Expr)> + 'a> {
    if !attr.path().is_ident("sval") {
        return None;
    }

    let mut results = Vec::new();
    attr.parse_nested_meta(|meta| {
        let expr: Expr = match meta.value() {
            Ok(value) => value.parse()?,
            // If there isn't a value associated with the item
            // then use the boolean `true`
            Err(_) => syn::parse_quote!(true),
        };

        let path = meta.path;

        results.push((path, expr));

        Ok(())
    })
    .unwrap_or_else(|e| panic!("failed to parse attribute on {}: {}", ctxt, e));

    Some(results)
}
