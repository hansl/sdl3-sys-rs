use super::{
    DocComment, Expr, Ident, Kw_enum, Op, Parse, ParseErr, ParseRawRes, Span, WsAndComments,
};
use std::borrow::Cow;

pub struct Enum {
    span: Span,
    doc: Option<DocComment>,
    ident: Option<Ident>,
    variants: Vec<EnumVariant>,
}

pub struct EnumVariant {
    doc: Option<DocComment>,
    ident: Ident,
    expr: Option<Expr>,
}

impl Parse for Enum {
    fn desc() -> Cow<'static, str> {
        "enum".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        if let Some(enum_kw) = Kw_enum::try_parse(&mut rest)? {
            WsAndComments::try_parse(&mut rest)?;
            let ident = Ident::try_parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            Op::<'{'>::parse(&mut rest)?;
            let mut variants = Vec::new();
            loop {
                WsAndComments::try_parse(&mut rest)?;
                let v_doc = DocComment::try_parse(&mut rest)?;
                if let Some(v_ident) = Ident::try_parse(&mut rest)? {
                    WsAndComments::try_parse(&mut rest)?;
                    let expr = if <Op![=]>::try_parse(&mut rest)?.is_some() {
                        WsAndComments::try_parse(&mut rest)?;
                        let expr = Expr::parse(&mut rest)?;
                        WsAndComments::try_parse(&mut rest)?;
                        Some(expr)
                    } else {
                        None
                    };
                    let got_comma = <Op![,]>::try_parse(&mut rest)?.is_some();
                    let v_doc = DocComment::try_parse_combine_postfix(v_doc, &mut rest)?;
                    variants.push(EnumVariant {
                        doc: v_doc,
                        ident: v_ident,
                        expr,
                    });
                    if !got_comma {
                        break;
                    }
                } else if let Some(v_doc) = v_doc {
                    return Err(ParseErr::new(
                        v_doc.span,
                        "trailing prefix doc comment in enum",
                    ));
                } else {
                    break;
                }
            }
            WsAndComments::try_parse(&mut rest)?;
            let closing_brace = Op::<'}'>::parse(&mut rest)?;
            let span = doc
                .as_ref()
                .map(|dc| dc.span.clone())
                .unwrap_or(enum_kw.span)
                .join(&closing_brace.span);
            Ok((
                rest,
                Some(Self {
                    span,
                    doc,
                    ident,
                    variants,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
