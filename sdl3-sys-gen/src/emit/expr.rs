use super::{patch_emit_macro_call, DefineState, Emit, EmitContext, EmitErr, EmitResult, Eval};
use crate::parse::{
    Alternative, Ambiguous, BinaryOp, DefineValue, Expr, FloatLiteral, FnCall, GetSpan, Ident,
    IntegerLiteral, IntegerLiteralType, Literal, Op, Parenthesized, ParseErr, PrimitiveType,
    RustCode, SizeOf, Span, StringLiteral, Type, TypeEnum,
};
use core::fmt::{self, Display, Write};

#[derive(Clone, Debug)]
pub enum Value {
    I32(i32),
    U31(u32),
    U32(u32),
    I64(i64),
    U63(u64),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(StringLiteral),
    TargetDependent(DefineState),
    RustCode(Box<RustCode>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            &Value::I32(i) => i != 0,
            &Value::U31(u) => u != 0,
            &Value::U32(u) => u != 0,
            &Value::I64(i) => i != 0,
            &Value::U63(u) => u != 0,
            &Value::U64(u) => u != 0,
            &Value::F32(f) => !f.is_nan() && f != 0.0,
            &Value::F64(f) => !f.is_nan() && f != 0.0,
            &Value::Bool(b) => b,
            Value::String(_) => true,
            Value::TargetDependent(_) | Value::RustCode(_) => unreachable!(),
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn ty(&self) -> Type {
        match self {
            Value::I32(_) => Type::primitive(PrimitiveType::Int32T),
            Value::U31(_) => Type::primitive(PrimitiveType::Int32T),
            Value::U32(_) => Type::primitive(PrimitiveType::Uint32T),
            Value::I64(_) => Type::primitive(PrimitiveType::Int64T),
            Value::U63(_) => Type::primitive(PrimitiveType::Int64T),
            Value::U64(_) => Type::primitive(PrimitiveType::Uint64T),
            Value::F32(_) => Type::primitive(PrimitiveType::Float),
            Value::F64(_) => Type::primitive(PrimitiveType::Double),
            Value::Bool(_) => Type::primitive(PrimitiveType::Bool),
            Value::String(_) => Type::rust("&::core::ffi::CStr", true),
            Value::RustCode(r) => r.ty.clone(),
            Value::TargetDependent(_) => todo!(),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::I32(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err(())
                }
            }
            Value::U31(u) => Ok(u as u64),
            Value::U32(u) => Ok(u as u64),
            Value::I64(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err(())
                }
            }
            Value::U63(u) => Ok(u),
            Value::U64(u) => Ok(u),
            _ => Err(()),
        }
    }
}

impl Emit for Value {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Value::I32(i) => write!(ctx, "{i}_i32")?,
            Value::U31(u) => write!(ctx, "{u}")?,
            Value::U32(u) => write!(ctx, "{u}_u32")?,
            Value::I64(i) => write!(ctx, "{i}_i64")?,
            Value::U63(i) => write!(ctx, "{i}_i64")?,
            Value::U64(u) => write!(ctx, "{u}_u64")?,
            &Value::F32(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    let dec = if s.contains('.') { "" } else { ".0" };
                    write!(ctx, "{s}{dec}_f32")?
                } else {
                    todo!()
                }
            }
            &Value::F64(f) => {
                let s = format!("{}", f);
                if s.parse() == Ok(f) {
                    let dec = if s.contains('.') { "" } else { ".0" };
                    write!(ctx, "{s}{dec}_f64")?
                } else {
                    todo!()
                }
            }
            &Value::Bool(b) => write!(ctx, "{b}")?,
            Value::String(s) => return s.emit(ctx),
            Value::TargetDependent(_) => {
                return Err(ParseErr::new(Span::none(), "can't emit indeterminate value").into())
            }
            Value::RustCode(s) => ctx.write_str(&s.value)?,
        }
        Ok(())
    }
}

impl Display for RustCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl Eval for DefineValue {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Self::Expr(expr) => expr.try_eval(ctx),
            Self::Ambiguous(amb) => amb.try_eval(ctx),
            Self::RustCode(r) => Ok(Some(Value::RustCode(r.clone()))),
            Self::TargetDependent | Self::Type(_) => Ok(None),
            Self::Empty => Ok(None),
            _ => {
                dbg!(self);
                todo!()
            }
        }
    }
}

impl Eval for Ambiguous {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        for alt in self.alternatives.iter() {
            if let Alternative::Expr(expr) = alt {
                if let Ok(Some(value)) = expr.try_eval(ctx) {
                    return Ok(Some(value));
                }
            }
        }
        Ok(None)
    }
}

impl Eval for IntegerLiteral {
    fn try_eval(&self, _ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self.kind {
            IntegerLiteralType::Unsuffixed => {
                if self.value <= i32::MAX as u64 {
                    Ok(Some(Value::U31(self.value as u32)))
                } else if self.value <= u32::MAX as u64 {
                    Ok(Some(Value::U32(self.value as u32)))
                } else if self.value <= i64::MAX as u64 {
                    Ok(Some(Value::U63(self.value)))
                } else {
                    Ok(Some(Value::U64(self.value)))
                }
            }
            IntegerLiteralType::Unsigned => {
                if self.value <= u32::MAX as u64 {
                    Ok(Some(Value::U32(self.value as u32)))
                } else {
                    todo!()
                }
            }
            IntegerLiteralType::Long => {
                // !!! FIXME: long can be 32-bit or 64-bit depending on platform
                if self.value <= i32::MAX as u64 {
                    Ok(Some(Value::I32(self.value as i32)))
                } else if self.value <= i64::MAX as u64 {
                    Ok(Some(Value::I64(self.value as i64)))
                } else {
                    todo!()
                }
            }
            IntegerLiteralType::UnsignedLong => todo!(),
            IntegerLiteralType::LongLong => Ok(Some(Value::I64(self.value as i64))),
            IntegerLiteralType::UnsignedLongLong => Ok(Some(Value::U64(self.value))),
        }
    }
}

impl Eval for SizeOf {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            SizeOf::Type(span, ty) => match &ty.ty {
                TypeEnum::Primitive(_) | TypeEnum::Pointer(_) => {
                    let out = ctx.capture_output(|ctx| {
                        write!(ctx, "::core::mem::size_of::<")?;
                        ty.emit(ctx)?;
                        write!(ctx, ">()")?;
                        Ok(())
                    })?;
                    Ok(Some(Value::RustCode(Box::new(RustCode {
                        value: out,
                        ty: Type::primitive(PrimitiveType::SizeT),
                    }))))
                }

                TypeEnum::Ident(ident) => {
                    if ctx.lookup_sym(ident).is_some() {
                        Ok(Some(Value::RustCode(Box::new(RustCode {
                            value: format!("::core::mem::size_of::<{}>()", ident.as_str()),
                            ty: Type::primitive(PrimitiveType::SizeT),
                        }))))
                    } else {
                        todo!()
                    }
                }

                TypeEnum::Infer(i) => {
                    if let Some(ty) = &*i.borrow() {
                        SizeOf::Type(span.clone(), ty.clone()).try_eval(ctx)
                    } else {
                        Ok(None)
                    }
                }

                TypeEnum::Enum(_) => todo!(),
                TypeEnum::Struct(_) => todo!(),
                TypeEnum::Array(_, _) => todo!(),
                TypeEnum::FnPointer(_) => todo!(),
                TypeEnum::DotDotDot => todo!(),
                TypeEnum::Rust(_, _) => todo!(),
                TypeEnum::Function(_) => todo!(),
            },

            SizeOf::Expr(_, Expr::Parenthesized(p)) => {
                SizeOf::Expr(p.span(), p.expr.clone()).try_eval(ctx)
            }

            SizeOf::Expr(_, expr) => {
                if let Expr::BinaryOp(bop) = &expr {
                    // special case: sizeof(((Type*)_)->field)
                    if bop.op.as_str() == "->" {
                        let lhs = if let Expr::Parenthesized(lhs) = &bop.lhs {
                            &lhs.expr
                        } else {
                            &bop.lhs
                        };
                        if let (Expr::Cast(lhs), Expr::Ident(rhs)) = (&lhs, &bop.rhs) {
                            if let Some(ty) = lhs.ty.get_pointer_type() {
                                let code = ctx.capture_output(|ctx| {
                                    write!(ctx, "size_of_field!(")?;
                                    ty.emit(ctx)?;
                                    write!(ctx, ", {rhs})")?;
                                    Ok(())
                                })?;
                                return Ok(Some(Value::RustCode(RustCode::boxed(
                                    code,
                                    Type::primitive(PrimitiveType::SizeT),
                                ))));
                            }
                        }
                    }
                }

                // !!! FIXME: size_of_val isn't const, so this'd require finding the type of the expression.
                // skip it for now
                Ok(None)
            }
        }
    }
}

impl Expr {
    pub fn try_eval_plus_one(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        Expr::BinaryOp(Box::new(BinaryOp {
            op: Op {
                span: Span::new_inline("+"),
            },
            lhs: self.clone(),
            rhs: Expr::Literal(Literal::Integer(IntegerLiteral::one())),
        }))
        .try_eval(ctx)
    }
}

impl Eval for Expr {
    fn try_eval(&self, ctx: &EmitContext) -> Result<Option<Value>, EmitErr> {
        match self {
            Expr::Parenthesized(p) => return p.expr.try_eval(ctx),

            Expr::Ident(ident) => {
                if let Ok(i) = ident.clone().try_into() {
                    if let Ok(Some((args, value))) = ctx.preproc_state().borrow().lookup(&i) {
                        if args.is_some() {
                            return Err(ParseErr::new(
                                self.span(),
                                "define with arguments used without arguments",
                            )
                            .into());
                        }
                        return value.try_eval(ctx);
                    } else if ctx.is_preproc_eval_mode() {
                        return Err(ParseErr::new(ident.span(), "undefined macro").into());
                    }
                }

                match ident.as_str() {
                    "true" => return Ok(Some(Value::Bool(true))),
                    "false" => return Ok(Some(Value::Bool(false))),
                    "size_t" => return Ok(None),
                    _ => {
                        if let Ok(ident) = Ident::try_from(ident.clone()) {
                            if let Some(sym) = ctx.lookup_sym(&ident) {
                                return if let Some(ty) = &sym.ty {
                                    Ok(Some(Value::RustCode(RustCode::boxed(
                                        ident.to_string(),
                                        ty.clone(),
                                    ))))
                                } else {
                                    Ok(None)
                                };
                            }
                        }
                    }
                }
            }

            Expr::Literal(lit) => {
                return match lit {
                    Literal::Integer(i) => i.try_eval(ctx),
                    Literal::Float(f) => match f {
                        FloatLiteral::Float(f) => Ok(Some(Value::F32(f.value))),
                        FloatLiteral::Double(f) => Ok(Some(Value::F64(f.value))),
                    },
                    Literal::String(s) => Ok(Some(Value::String(s.clone()))),
                }
            }

            Expr::FnCall(call) => {
                if ctx.is_preproc_eval_mode() {
                    if let Expr::Ident(ident) = &*call.func {
                        match ident.as_str() {
                            "defined" => {
                                let args = &*call.args;
                                let err = || {
                                    Err(ParseErr::new(
                                        call.span(),
                                        "defined() in #if takes one argument of type ident",
                                    )
                                    .into())
                                };
                                if args.len() != 1 {
                                    return err();
                                }
                                let Expr::Ident(define) = &args[0] else {
                                    return err();
                                };
                                let define = define.clone().try_into()?;
                                return if ctx.preproc_state().borrow().is_target_define(&define) {
                                    Ok(Some(Value::TargetDependent(DefineState::defined(define))))
                                } else if ctx.preproc_state().borrow().is_defined(&define)? {
                                    Ok(Some(Value::Bool(true)))
                                } else {
                                    Ok(Some(Value::Bool(false)))
                                };
                            }

                            "SDL_HAS_BUILTIN" => {
                                let args = &*call.args;
                                let err = || {
                                    Err(ParseErr::new(
                                        call.span(),
                                        "SDL_HAS_BUILTIN takes one argument of type ident",
                                    )
                                    .into())
                                };
                                if args.len() != 1 {
                                    return err();
                                }
                                let Expr::Ident(builtin) = &args[0] else {
                                    return err();
                                };
                                return Ok(Some(Value::Bool(match builtin.as_str() {
                                    "__builtin_add_overflow" | "__builtin_mul_overflow" => true,
                                    _ => {
                                        return Err(ParseErr::new(
                                            builtin.span(),
                                            "unknown builtin",
                                        )
                                        .into())
                                    }
                                })));
                            }

                            _ => (),
                        }
                    }
                } else {
                    // not preproc

                    if let Expr::Ident(ident) = &*call.func {
                        match ident.as_str() {
                            "SDL_SINT64_C" | "SDL_UINT64_C" => {
                                let args = &*call.args;
                                let err = || {
                                    Err(ParseErr::new(call.span(), "expected one argument").into())
                                };
                                if args.len() != 1 {
                                    return err();
                                }
                                let Some(arg) = args[0].try_eval(ctx)? else {
                                    return Ok(None);
                                };
                                return Ok(Some(match ident.as_str() {
                                    "SDL_SINT64_C" => match arg {
                                        Value::I32(i) => Value::I64(i as i64),
                                        Value::U31(u) | Value::U32(u) => Value::I64(u as i64),
                                        Value::I64(i) => Value::I64(i),
                                        Value::U63(u) | Value::U64(u) => Value::I64(u as i64),
                                        _ => todo!(),
                                    },
                                    "SDL_UINT64_C" => match arg {
                                        Value::I32(i) => Value::U64(i as u64),
                                        Value::U31(u) | Value::U32(u) => Value::U64(u as u64),
                                        Value::I64(i) => Value::U64(i as u64),
                                        Value::U63(u) | Value::U64(u) => Value::U64(u),
                                        _ => todo!(),
                                    },
                                    _ => unreachable!(),
                                }));
                            }

                            _ => (),
                        }
                    }

                    return Ok(None); // !!! FIXME
                }
            }

            Expr::Ambiguous(amb) => {
                let mut result = None;
                for alt in amb.alternatives.iter() {
                    if let Alternative::Expr(expr) = alt {
                        if let Some(value) = expr.try_eval(ctx)? {
                            if result.is_none() {
                                result = Some(value.clone());
                            } else {
                                return Ok(None);
                            }
                        }
                    }
                }
                return Ok(result);
            }

            Expr::Cast(cast) => {
                let out = ctx.capture_output(|ctx| {
                    cast.expr.emit(ctx)?;
                    write!(ctx, " as ")?;
                    cast.ty.emit(ctx)
                })?;
                return Ok(Some(Value::RustCode(Box::new(RustCode {
                    value: out,
                    ty: cast.ty.clone(),
                }))));
            }

            Expr::Asm(_) => return Ok(None),

            Expr::SizeOf(s) => return s.try_eval(ctx),

            Expr::UnaryOp(uop) => {
                let expr = uop.expr.try_eval(ctx)?;
                let Some(expr) = expr else { return Ok(None) };

                return match uop.op.as_str().as_bytes() {
                    b"!" => match expr {
                        Value::String(_) => Ok(None),
                        Value::TargetDependent(ds) => Ok(Some(Value::TargetDependent(ds.not()))),
                        _ => Ok(Some(Value::Bool(expr.is_falsy()))),
                    },

                    b"+" => match expr {
                        Value::String(_) => Ok(None),
                        _ => Ok(Some(expr)),
                    },

                    b"-" => match expr {
                        Value::I32(value) => {
                            Ok(Some(Value::I32(value.checked_neg().ok_or_else(|| {
                                ParseErr::new(uop.span(), "can't negate i32::MIN")
                            })?)))
                        }
                        Value::U31(value) | Value::U32(value) => {
                            Ok(Some(Value::I32(-(value as i32))))
                        }
                        Value::I64(value) => {
                            Ok(Some(Value::I64(value.checked_neg().ok_or_else(|| {
                                ParseErr::new(uop.span(), "can't negate i64::MIN")
                            })?)))
                        }
                        Value::U63(value) | Value::U64(value) => {
                            Ok(Some(Value::I64(-(value as i64))))
                        }
                        Value::F32(value) => Ok(Some(Value::F32(-value))),
                        Value::F64(value) => Ok(Some(Value::F64(-value))),
                        Value::TargetDependent(_) => Ok(Some(expr)),
                        _ => Ok(None),
                    },

                    b"~" => match expr {
                        Value::I32(value) => Ok(Some(Value::I32(!value))),
                        Value::U31(value) => Ok(Some(Value::I32(!(value as i32)))),
                        Value::U32(value) => Ok(Some(Value::U32(!value))),
                        Value::I64(value) => Ok(Some(Value::I64(!value))),
                        Value::U63(value) => Ok(Some(Value::I64(!(value as i64)))),
                        Value::U64(value) => Ok(Some(Value::U64(!value))),
                        Value::RustCode(c) => Ok(Some(Value::RustCode(RustCode::boxed(
                            format!("!({})", c.value),
                            c.ty,
                        )))),
                        _ => todo!(),
                    },

                    _ => Err(ParseErr::new(uop.span(), "missing implementation for eval").into()),
                };
            }

            Expr::BinaryOp(bop) => {
                macro_rules! eval {
                    ($expr:expr) => {{
                        let Some(value) = $expr.try_eval(ctx)? else {
                            return Ok(None);
                        };
                        value
                    }};
                }

                macro_rules! bitop {
                    ($op:tt) => {
                        match (eval!(bop.lhs), eval!(bop.rhs)) {
                            (Value::I32(lhs), Value::I32(rhs)) => Ok(Some(Value::I32(lhs $op rhs))),
                            (Value::I32(lhs), Value::U31(rhs)) => Ok(Some(Value::I32(lhs $op rhs as i32))),
                            (Value::U31(lhs), Value::I32(rhs)) => Ok(Some(Value::I32(lhs as i32 $op rhs))),
                            (Value::U31(lhs), Value::U31(rhs)) => Ok(Some(Value::U31(lhs $op rhs))),
                            (Value::U31(lhs) | Value::U32(lhs), Value::U31(rhs) | Value::U32(rhs)) => Ok(Some(Value::U32(lhs $op rhs))),
                            (Value::RustCode(lhs), Value::RustCode(rhs)) if lhs.ty == rhs.ty => Ok(Some(Value::RustCode(RustCode::boxed(
                                format!("({} {} {})", lhs.value, stringify!($op), rhs.value),
                                lhs.ty
                            )))),
                            x => {dbg!(x);todo!()},
                        }
                    };
                }

                macro_rules! calc {
                    ($op:tt) => {
                        match (eval!(bop.lhs), eval!(bop.rhs)) {
                            (Value::I32(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::I32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs, rhs as i32)))))
                            }
                            (Value::U31(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::I32(calc!(@ checked $op (lhs as i32, rhs)))))
                            }
                            (Value::U31(lhs), Value::U31(rhs)) => {
                                let value = calc!(@ checked $op (lhs, rhs));
                                if value <= i32::MAX as u32 {
                                    Ok(Some(Value::U31(value)))
                                } else {
                                    Ok(Some(Value::U32(value)))
                                }
                            }
                            (Value::U31(lhs), Value::U32(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::U32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::U32(lhs), Value::U32(rhs)) => {
                                Ok(Some(Value::U32(calc!(@ checked $op (lhs, rhs)))))
                            }
                            (Value::F32(lhs), Value::F32(rhs)) => Ok(Some(Value::F32(lhs $op rhs))),
                            (Value::F64(lhs), Value::F64(rhs)) => Ok(Some(Value::F64(lhs $op rhs))),

                            (Value::RustCode(_), _) => {
                                // FIXME
                                Ok(None)
                            }

                            (Value::U31(lhs), Value::RustCode(rhs)) => {
                                // FIXME
                                assert!(matches!(rhs.ty.ty, TypeEnum::Primitive(PrimitiveType::SizeT)));
                                Ok(Some(Value::RustCode(RustCode::boxed(format!("({} {} {})", lhs, stringify!($op), rhs.value), rhs.ty))))
                            }

                            _ => Err(ParseErr::new(bop.span(), "missing implementation for eval").into()),
                        }
                    };

                    (@ checked * ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_mul($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked + ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_add($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };

                    (@ checked - ($lhs:expr, $rhs:expr)) => {
                        $lhs.checked_add($rhs).ok_or_else(|| {
                            ParseErr::new(bop.span(), "evaluated value out of range")
                        })?
                    };
                }

                macro_rules! compare {
                    ($op:tt) => {{
                        let op = stringify!($op);
                        match (eval!(bop.lhs), eval!(bop.rhs)) {
                            (Value::I32(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::I32(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs as i32) as u32)))
                            }
                            (Value::U31(lhs), Value::I32(rhs)) => {
                                Ok(Some(Value::U31(((lhs as i32) $op rhs) as u32)))
                            }
                            (Value::U31(lhs), Value::U31(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::F32(lhs), Value::F32(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::F64(lhs), Value::F64(rhs)) => {
                                Ok(Some(Value::U31((lhs $op rhs) as u32)))
                            }
                            (Value::RustCode(lhs), rhs) => {
                                let code = ctx.capture_output(|ctx| {
                                    write!(ctx, "{lhs} {op} ")?;
                                    rhs.emit( ctx)
                                })?;
                                Ok(Some(Value::RustCode(Box::new(RustCode{ value:code, ty:Type::primitive(PrimitiveType::Bool) }))))
                            }
                            _ => Err(ParseErr::new(bop.span(), format!("invalid operands to `{op}`")).into()),
                        }
                    }};
                }

                macro_rules! shift {
                    ($op:tt) => {{
                        let op = stringify!($op);
                        let Ok(shift) = u64::try_from(eval!(bop.rhs)) else {
                            return Err(ParseErr::new(
                                bop.rhs.span(),
                                format!("invalid shift value"),
                            )
                            .into());
                        };

                        match eval!(bop.lhs) {
                            Value::U32(lhs) => Ok(Some(Value::U32(lhs $op shift))),

                            _ => Err(ParseErr::new(
                                bop.lhs.span(),
                                format!("invalid operand to `{op}`"),
                            )
                            .into()),
                        }
                    }};
                }

                return match bop.op.as_str().as_bytes() {
                    b"*" => calc!(*),
                    b"+" => calc!(+),
                    b"-" => calc!(-),

                    b"&" => bitop!(&),
                    b"|" => bitop!(|),

                    b"," => {
                        let _lhs = eval!(bop.lhs);
                        let rhs = eval!(bop.rhs);
                        Ok(Some(rhs))
                    }

                    b"&&" => {
                        let lhs = eval!(bop.lhs);
                        if let Value::TargetDependent(lhs) = lhs {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(lhs.all(rhs))))
                            } else if rhs.is_truthy() {
                                Ok(Some(Value::TargetDependent(lhs)))
                            } else {
                                Ok(Some(Value::Bool(false)))
                            }
                        } else if let Value::RustCode(_) = lhs {
                            let rhs = eval!(bop.rhs);
                            let out = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " && ")?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                out,
                                Type::primitive(PrimitiveType::Bool),
                            ))))
                        } else if lhs.is_truthy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::Bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::Bool(false)))
                        }
                    }

                    b"||" => {
                        let lhs = eval!(bop.lhs);
                        if let Value::TargetDependent(lhs) = lhs {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(lhs.any(rhs))))
                            } else if rhs.is_falsy() {
                                Ok(Some(Value::TargetDependent(lhs)))
                            } else {
                                Ok(Some(Value::Bool(true)))
                            }
                        } else if let Value::RustCode(_) = lhs {
                            let rhs = eval!(bop.rhs);
                            let out = ctx.capture_output(|ctx| {
                                write!(ctx, "(")?;
                                lhs.emit(ctx)?;
                                write!(ctx, " || ")?;
                                rhs.emit(ctx)?;
                                write!(ctx, ")")?;
                                Ok(())
                            })?;
                            Ok(Some(Value::RustCode(RustCode::boxed(
                                out,
                                Type::primitive(PrimitiveType::Bool),
                            ))))
                        } else if lhs.is_falsy() {
                            let rhs = eval!(bop.rhs);
                            if let Value::TargetDependent(rhs) = rhs {
                                Ok(Some(Value::TargetDependent(rhs)))
                            } else {
                                Ok(Some(Value::Bool(rhs.is_truthy())))
                            }
                        } else {
                            // short circuit
                            Ok(Some(Value::Bool(true)))
                        }
                    }

                    b"==" => compare!(==),
                    b"!=" => compare!(!=),
                    b"<" => compare!(<),
                    b"<=" => compare!(<=),
                    b">" => compare!(>),
                    b">=" => compare!(>=),

                    b"<<" => shift!(<<),
                    b">>" => shift!(>>),

                    _ => {
                        return Err(
                            ParseErr::new(bop.span(), "missing implementation for eval").into()
                        )
                    }
                };
            }

            Expr::PostOp(_) => (),
            Expr::Ternary(_) => (),
            Expr::ArrayIndex { .. } => (),
            Expr::ArrayValues { .. } => (),

            Expr::HasInclude(_) => return Ok(Some(Value::Bool(false))),

            Expr::Value(value) => return Ok(Some(value.clone())),
        }

        Err(ParseErr::new(self.span(), "missing implementation for eval").into())
    }
}

impl Emit for Expr {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Expr::Parenthesized(e) => e.emit(ctx),
            Expr::Ident(i) => {
                if let Some(sym) = ctx.lookup_sym(&i.clone().try_into().unwrap()) {
                    sym.ident.emit(ctx)
                } else {
                    Err(ParseErr::new(i.span(), "undefined symbol").into())
                }
            }
            Expr::Literal(lit) => lit.emit(ctx),
            Expr::FnCall(call) => call.emit(ctx),
            Expr::Ambiguous(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::Asm(_) => todo!(),
            Expr::SizeOf(_) => todo!(),

            Expr::UnaryOp(uop) => {
                if let Some(value) = self.try_eval(ctx)? {
                    value.emit(ctx)
                } else {
                    match uop.op.as_str().as_bytes() {
                        b"+" => uop.expr.emit(ctx),
                        b"-" => {
                            write!(ctx, "-(")?;
                            uop.expr.emit(ctx)?;
                            write!(ctx, ")")?;
                            Ok(())
                        }
                        _ => todo!(),
                    }
                }
            }

            Expr::BinaryOp(_) => {
                if let Some(value) = self.try_eval(ctx)? {
                    value.emit(ctx)
                } else {
                    todo!()
                }
            }

            Expr::PostOp(_) => todo!(),
            Expr::Ternary(_) => todo!(),
            Expr::ArrayIndex { .. } => todo!(),
            Expr::ArrayValues { .. } => todo!(),
            Expr::Value(value) => value.emit(ctx),
            Expr::HasInclude(_) => todo!(),
        }
    }
}

impl Emit for Parenthesized {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        ctx.write_char('(')?;
        self.expr.emit(ctx)?;
        ctx.write_char(')')?;
        Ok(())
    }
}

impl Emit for FnCall {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        if let Expr::Ident(ident) = &*self.func {
            if patch_emit_macro_call(ctx, ident.as_str(), &self.args)? {
                Ok(())
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Emit for Literal {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            Literal::Integer(lit) => lit.emit(ctx),
            Literal::Float(lit) => lit.emit(ctx),
            Literal::String(_) => todo!(),
        }
    }
}

impl Emit for IntegerLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        let (pfx, mut s) = match self.base {
            2 => ("0b", format!("{:b}", self.value)),
            8 => ("0o", format!("{:o}", self.value)),
            10 => ("", format!("{}", self.value)),
            16 => ("0x", format!("{:x}", self.value)),
            _ => return Err(ParseErr::new(self.span(), "can't emit base").into()),
        };
        if s.len() < self.ndigits as usize {
            s = format!("{s}{}", "0".repeat(self.ndigits as usize - s.len()));
        }
        write!(ctx, "{}{}", pfx, s)?;
        Ok(())
    }
}

impl Emit for FloatLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        match self {
            FloatLiteral::Float(f) => Value::F32(f.value).emit(ctx),
            FloatLiteral::Double(f) => Value::F64(f.value).emit(ctx),
        }
    }
}

impl Emit for StringLiteral {
    fn emit(&self, ctx: &mut EmitContext) -> EmitResult {
        write!(
            ctx,
            "unsafe {{ ::core::ffi::CStr::from_bytes_with_nul_unchecked(b\""
        )?;
        for &b in self.str.as_bytes() {
            match b {
                0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7f => ctx.write_char(b as char)?,
                b'\"' | b'\\' => {
                    ctx.write_char('\\')?;
                    ctx.write_char(b as char)?
                }
                _ => write!(ctx, "\\x{:02x}", b)?,
            }
        }
        write!(ctx, "\\0\") }}")?;
        Ok(())
    }
}
