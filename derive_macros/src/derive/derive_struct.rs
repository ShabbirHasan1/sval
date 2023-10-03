use syn::{Attribute, Fields, Generics, Ident, Path};

use crate::{
    attr, bound,
    derive::impl_tokens,
    index::{Index, IndexAllocator, IndexValue},
    label::{label_or_ident, LabelValue},
    stream::{stream_record_tuple, RecordTupleTarget},
    tag::quote_optional_tag_owned,
};

pub(crate) struct StructAttrs {
    tag: Option<Path>,
    label: Option<LabelValue>,
    index: Option<IndexValue>,
    unlabeled_fields: bool,
    unindexed_fields: bool,
}

impl StructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::check(
            "struct",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::UnlabeledFieldsAttr,
                &attr::UnindexedFieldsAttr,
            ],
            attrs,
        );

        let tag = attr::get_unchecked("struct", attr::TagAttr, attrs);
        let label = attr::get_unchecked("struct", attr::LabelAttr, attrs);
        let index = attr::get_unchecked("struct", attr::IndexAttr, attrs);

        let unlabeled_fields =
            attr::get_unchecked("struct", attr::UnlabeledFieldsAttr, attrs).unwrap_or(false);
        let unindexed_fields =
            attr::get_unchecked("struct", attr::UnindexedFieldsAttr, attrs).unwrap_or(false);

        StructAttrs {
            tag,
            label,
            index,
            unlabeled_fields,
            unindexed_fields,
        }
    }

    pub(crate) fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    pub(crate) fn label(&self) -> Option<LabelValue> {
        self.label.clone()
    }

    pub(crate) fn index(&self) -> Option<Index> {
        self.index.clone().map(IndexAllocator::const_index_of)
    }

    pub(crate) fn unlabeled_fields(&self) -> bool {
        self.unlabeled_fields
    }

    pub(crate) fn unindexed_fields(&self) -> bool {
        self.unindexed_fields
    }
}

pub(crate) fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: &Fields,
    attrs: &StructAttrs,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let (fields, target) = match fields {
        Fields::Named(ref fields) => (&fields.named, RecordTupleTarget::named_fields()),
        Fields::Unnamed(ref fields) => (&fields.unnamed, RecordTupleTarget::unnamed_fields()),
        _ => unreachable!(),
    };

    let match_arm = stream_record_tuple(
        quote!(#ident),
        fields.iter(),
        target,
        attrs.tag(),
        Some(label_or_ident(attrs.label(), ident)),
        attrs.index(),
        attrs.unlabeled_fields(),
        attrs.unindexed_fields(),
    );

    let tag = quote_optional_tag_owned(attrs.tag());

    impl_tokens(
        impl_generics,
        ident,
        ty_generics,
        &bounded_where_clause,
        quote!({
            match self {
                #match_arm
            }

            Ok(())
        }),
        Some(tag),
    )
}
