use super::{
    DocComment, Enum, Ident, Kw_const, Kw_typedef, Op, Parse, ParseRawRes, PrimitiveType,
    PrimitiveTypeParse, Punctuated, Span, StructOrUnion, UintLiteral, VarDecl, WsAndComments,
};
use std::borrow::Cow;

pub struct Type {
    span: Span,
    is_const: bool,
    ty: TypeEnum,
}

impl Parse for Type {
    fn desc() -> Cow<'static, str> {
        "type".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (rest, Some(ty)) = TypeWithNoIdent::try_parse_raw(input)? {
            Ok((rest, Some(ty.ty)))
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub enum TypeEnum {
    Primitive(PrimitiveType),
    Ident(Ident),
    Enum(Box<Enum>),
    Struct(Box<StructOrUnion>),
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    FnPointer(Box<FnPointerType>),
}

pub struct FnPointerType {
    return_type: Type,
    args: Punctuated<VarDecl, Op![,]>,
}

const NO_IDENT: u8 = 0;
const OPT_IDENT: u8 = 1;
const REQ_IDENT: u8 = 2;

pub type TypeWithNoIdent = TypeWithIdent<NO_IDENT>;
pub type TypeWithOptIdent = TypeWithIdent<OPT_IDENT>;
pub type TypeWithReqIdent = TypeWithIdent<REQ_IDENT>;

pub struct TypeWithIdent<const IDENT_SPEC: u8> {
    pub ty: Type,
    pub ident: Option<Ident>,
}

impl<const IDENT_SPEC: u8> Parse for TypeWithIdent<IDENT_SPEC> {
    fn desc() -> Cow<'static, str> {
        "type".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let mut ty = if let Some(PrimitiveTypeParse { span, ty, is_const }) =
            PrimitiveTypeParse::try_parse(&mut rest)?
        {
            Type {
                span,
                is_const,
                ty: TypeEnum::Primitive(ty),
            }
        } else {
            let mut rest2 = rest.clone();
            let is_const = Kw_const::try_parse(&mut rest2)?.is_some();
            WsAndComments::try_parse(&mut rest2)?;
            if let Some(e) = Enum::try_parse(&mut rest2)? {
                rest = rest2;
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Enum(Box::new(e)),
                }
            } else if let Some(s) = StructOrUnion::try_parse(&mut rest2)? {
                rest = rest2;
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Struct(Box::new(s)),
                }
            } else if let Some(ident) = Ident::try_parse(&mut rest2)? {
                rest = rest2;
                let is_const = is_const || Kw_const::try_parse(&mut rest)?.is_some();
                let span = input.start().join(&rest.start());
                Type {
                    span,
                    is_const,
                    ty: TypeEnum::Ident(ident),
                }
            } else {
                return Ok((input.clone(), None));
            }
        };

        // pointer
        let mut rest2 = rest.clone();
        WsAndComments::try_parse(&mut rest2)?;
        while <Op![*]>::try_parse(&mut rest2)?.is_some() {
            rest = rest2.clone();
            WsAndComments::try_parse(&mut rest2)?;
            let is_const = Kw_const::try_parse(&mut rest2)?.is_some();
            if is_const {
                rest = rest2.clone();
                WsAndComments::try_parse(&mut rest2)?;
            }
            let span = input.start().join(&rest.start());
            ty = Type {
                is_const,
                ty: TypeEnum::Pointer(Box::new(ty)),
                span,
            };
        }

        // prepare function pointer
        rest2 = rest.clone();
        WsAndComments::try_parse(&mut rest2)?;
        let (ident, args) = if Op::<'('>::try_parse(&mut rest2)?.is_some() {
            rest = rest2;
            WsAndComments::try_parse(&mut rest)?;
            <Op![*]>::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            let ident = if IDENT_SPEC == NO_IDENT {
                None
            } else if IDENT_SPEC == REQ_IDENT {
                Some(Ident::parse(&mut rest)?)
            } else {
                Ident::try_parse(&mut rest)?
            };
            WsAndComments::try_parse(&mut rest)?;
            Op::<')'>::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            Op::<'('>::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            let args = Punctuated::<VarDecl, Op![,]>::try_parse(&mut rest)?.unwrap_or_default();
            WsAndComments::try_parse(&mut rest)?;
            Op::<')'>::parse(&mut rest)?;
            (ident, Some(args))
        } else {
            let ident = if IDENT_SPEC == NO_IDENT {
                None
            } else if IDENT_SPEC == REQ_IDENT {
                rest = rest2;
                Some(Ident::parse(&mut rest)?)
            } else {
                let ident = Ident::try_parse(&mut rest2)?;
                if ident.is_some() {
                    rest = rest2;
                }
                ident
            };
            (ident, None)
        };

        // array
        if ident.is_some() {
            rest2 = rest.clone();
            WsAndComments::try_parse(&mut rest2)?;
            while Op::<'['>::try_parse(&mut rest2)?.is_some() {
                rest = rest2;
                WsAndComments::try_parse(&mut rest)?;
                if let Some(size) = UintLiteral::try_parse(&mut rest)? {
                    WsAndComments::try_parse(&mut rest)?;
                    let span = input.start().join(&rest.start());
                    ty = Type {
                        is_const: true,
                        ty: TypeEnum::Array(Box::new(ty), size.u64()? as usize),
                        span,
                    }
                } else {
                    let span = input.start().join(&rest.start());
                    ty = Type {
                        is_const: true,
                        ty: TypeEnum::Pointer(Box::new(ty)),
                        span,
                    }
                }
                Op::<']'>::parse(&mut rest)?;
                rest2 = rest.clone();
                WsAndComments::try_parse(&mut rest2)?;
            }
        }

        // finish function pointer
        if let Some(args) = args {
            let span = input.start().join(&rest.start());
            ty = Type {
                span,
                is_const: true,
                ty: TypeEnum::FnPointer(Box::new(FnPointerType {
                    return_type: ty,
                    args,
                })),
            };
        }

        Ok((rest, Some(Self { ty, ident })))
    }
}

pub struct TypeDef {
    span: Span,
    ident: Ident,
    ty: Type,
}

impl Parse for TypeDef {
    fn desc() -> Cow<'static, str> {
        "typedef".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let mut rest = input.clone();
        let doc = DocComment::try_parse(&mut rest)?;
        if let Some(typedef_kw) = Kw_typedef::try_parse(&mut rest)? {
            let TypeWithIdent { ty, ident } = TypeWithReqIdent::parse(&mut rest)?;
            WsAndComments::try_parse(&mut rest)?;
            let semi = <Op![;]>::parse(&mut rest)?;
            let span = doc
                .map(|dc| dc.span)
                .unwrap_or(typedef_kw.span)
                .join(&semi.span);
            Ok((
                rest,
                Some(TypeDef {
                    span,
                    ident: ident.unwrap(),
                    ty,
                }),
            ))
        } else {
            Ok((input.clone(), None))
        }
    }
}
