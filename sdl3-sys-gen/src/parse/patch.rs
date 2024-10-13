use super::{
    Cast, Define, DefineValue, Enum, Expr, GetSpan, ParseContext, ParseErr, PrimitiveType, Type,
};

struct Patch<T: ?Sized> {
    module: Option<&'static str>,
    match_ident: fn(&str) -> bool,
    patch: fn(&ParseContext, &mut T) -> Result<bool, ParseErr>,
}

pub fn patch_parsed_define(ctx: &ParseContext, define: &mut Define) -> Result<bool, ParseErr> {
    patch(ctx, define, |d| d.ident.as_str(), DEFINE_PATCHES)
}

type DefinePatch = Patch<Define>;

const DEFINE_PATCHES: &[Patch<Define>] = &[
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_AUDIO_BITSIZE",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioFormat");
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_AUDIO_FRAMESIZE",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioSpec");
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i.starts_with("SDL_AUDIO_IS"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_AudioFormat");
            define.value = define.value.cast_expr(Type::bool());
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_DEFINE_AUDIO_FORMAT",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::bool();
            args[1].ty = Type::bool();
            args[2].ty = Type::bool();
            args[3].ty = Type::primitive(PrimitiveType::Uint8T);
            define.value = define.value.cast_expr(Type::ident_str("SDL_AudioFormat"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("error"),
        match_ident: |i| i == "SDL_InvalidParamError",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::primitive(PrimitiveType::Char), true);
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("gpu"),
        match_ident: |i| i == "SDL_GPU_SHADERFORMAT_INVALID",
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint32"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("haptic"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_HAPTIC_CARTESIAN"
                    | "SDL_HAPTIC_POLAR"
                    | "SDL_HAPTIC_SPHERICAL"
                    | "SDL_HAPTIC_STEERING_AXIS"
            )
        },
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("haptic"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_HAPTIC_CONSTANT"
                    | "SDL_HAPTIC_CUSTOM"
                    | "SDL_HAPTIC_DAMPER"
                    | "SDL_HAPTIC_FRICTION"
                    | "SDL_HAPTIC_INERTIA"
                    | "SDL_HAPTIC_LEFTRIGHT"
                    | "SDL_HAPTIC_RAMP"
                    | "SDL_HAPTIC_RESERVED1"
                    | "SDL_HAPTIC_RESERVED2"
                    | "SDL_HAPTIC_RESERVED3"
                    | "SDL_HAPTIC_SAWTOOTHDOWN"
                    | "SDL_HAPTIC_SAWTOOTHUP"
                    | "SDL_HAPTIC_SINE"
                    | "SDL_HAPTIC_SPRING"
                    | "SDL_HAPTIC_SQUARE"
                    | "SDL_HAPTIC_TRIANGLE"
            )
        },
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint16"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("joystick"),
        match_ident: |i| i.starts_with("SDL_HAT_"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("joystick"),
        match_ident: |i| matches!(i, "SDL_JOYSTICK_AXIS_MAX" | "SDL_JOYSTICK_AXIS_MIN"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Sint16"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("keycode"),
        match_ident: |i| i == "SDL_SCANCODE_TO_KEYCODE",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_Scancode");
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("keycode"),
        match_ident: |i| i.starts_with("SDL_KMOD_"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("SDL_Keymod"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("mouse"),
        match_ident: |i| matches!(i, "SDL_BUTTON_MASK"),
        patch: |_ctx, define| {
            define.value = define
                .value
                .cast_expr(Type::ident_str("SDL_MouseButtonFlags"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("mutex"),
        match_ident: |_| true,
        patch: |_, define| {
            if let DefineValue::Expr(Expr::FnCall(call)) = &define.value {
                if let Expr::Ident(ident) = &*call.func {
                    if matches!(
                        ident.as_str(),
                        "__attribute__" | "SDL_THREAD_ANNOTATION_ATTRIBUTE__"
                    ) {
                        define.value = DefineValue::Empty;
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_ALPHA_OPAQUE" | "SDL_ALPHA_TRANSPARENT"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint8"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_BITSPERPIXEL" | "SDL_BYTESPERPIXEL"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define
                .value
                .cast_expr(Type::primitive(PrimitiveType::Uint8T));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| i.starts_with("SDL_COLORSPACE"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_Colorspace");
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_DEFINE_COLORSPACE"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_ColorType");
            args[1].ty = Type::ident_str("SDL_ColorRange");
            args[2].ty = Type::ident_str("SDL_ColorPrimaries");
            args[3].ty = Type::ident_str("SDL_TransferCharacteristics");
            args[4].ty = Type::ident_str("SDL_MatrixCoefficients");
            args[5].ty = Type::ident_str("SDL_ChromaLocation");
            define.value = define.value.cast_expr(Type::ident_str("SDL_Colorspace"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_DEFINE_PIXELFORMAT"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            assert!(args[0].ident.as_str() == "type");
            args[0].ty = Type::ident_str("SDL_PixelType");
            assert!(args[1].ident.as_str() == "order");
            //args[1].ty = !!! FIXME
            assert!(args[2].ident.as_str() == "layout");
            args[2].ty = Type::ident_str("SDL_PackedLayout");
            args[3].ty = Type::primitive(PrimitiveType::Uint8T);
            args[4].ty = Type::primitive(PrimitiveType::Uint8T);
            define.value = define.value.cast_expr(Type::ident_str("SDL_PixelFormat"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_PIXELFLAG" | "SDL_PIXELORDER"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_PIXELLAYOUT"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define.value.cast_expr(Type::ident_str("SDL_PackedLayout"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("pixels"),
        match_ident: |i| matches!(i, "SDL_PIXELTYPE"),
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::ident_str("SDL_PixelFormat");
            define.value = define.value.cast_expr(Type::ident_str("SDL_PixelType"));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("render"),
        match_ident: |i| i.starts_with("SDL_RENDERER_VSYNC_"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::primitive(PrimitiveType::Int));
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("stdinc"),
        match_ident: |i| i == "SDL_iconv_wchar_utf8",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::primitive(PrimitiveType::WcharT), true);
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("surface"),
        match_ident: |i| i == "SDL_MUSTLOCK",
        patch: |_ctx, define| {
            let args = define.args.as_mut().unwrap();
            args[0].ty = Type::pointer(Type::ident_str("SDL_Surface"), true);
            Ok(true)
        },
    },
    DefinePatch {
        module: Some("system"),
        match_ident: |i| i.starts_with("SDL_ANDROID_EXTERNAL_STORAGE_"),
        patch: |_ctx, define| {
            define.value = define.value.cast_expr(Type::ident_str("Uint32"));
            Ok(true)
        },
    },
];

pub fn patch_parsed_enum(ctx: &ParseContext, e: &mut Enum) -> Result<bool, ParseErr> {
    patch(
        ctx,
        e,
        |e| e.ident.as_ref().map(|i| i.as_str()).unwrap_or(""),
        ENUM_PATCHES,
    )
}

type EnumPatch = Patch<Enum>;

const ENUM_PATCHES: &[EnumPatch] = &[
    EnumPatch {
        module: Some("audio"),
        match_ident: |i| i == "SDL_AudioFormat",
        patch: |_, e| {
            e.base_type = Some(Type::primitive(PrimitiveType::UnsignedInt));
            Ok(true)
        },
    },
    EnumPatch {
        module: Some("events"),
        match_ident: |i| i == "SDL_EventType",
        patch: |_, e| {
            e.base_type = Some(Type::ident_str("Uint32"));
            Ok(true)
        },
    },
    EnumPatch {
        module: Some("pixels"),
        match_ident: |i| i == "SDL_Colorspace",
        patch: |_, e| {
            e.base_type = Some(Type::ident_str("Uint32"));
            Ok(true)
        },
    },
    EnumPatch {
        module: Some("pixels"),
        match_ident: |i| {
            matches!(
                i,
                "SDL_ChromaLocation"
                    | "SDL_ColorPrimaries"
                    | "SDL_ColorRange"
                    | "SDL_ColorType"
                    | "SDL_MatrixCoefficients"
                    | "SDL_TransferCharacteristics"
            )
        },
        patch: |_, e| {
            e.base_type = Some(Type::primitive(PrimitiveType::UnsignedInt));
            Ok(true)
        },
    },
];

pub fn patch_parsed_expr(_ctx: &ParseContext, expr: &mut Expr) -> Result<bool, ParseErr> {
    #[allow(clippy::single_match)]
    match expr {
        Expr::FnCall(f) => match &*f.func {
            Expr::Ident(i) => match i.as_str() {
                "SDL_const_cast" | "SDL_reinterpret_cast" | "SDL_static_cast" => {
                    let Expr::Ident(ty) = f.args[0].clone() else {
                        todo!()
                    };
                    *expr = Expr::Cast(Cast::boxed(
                        f.span(),
                        Type::ident(ty.try_into().unwrap()),
                        f.args[1].clone(),
                    ));
                    return Ok(true);
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
    Ok(false)
}

fn patch<T: ?Sized>(
    ctx: &ParseContext,
    parsed: &mut T,
    get_ident: impl Fn(&T) -> &str,
    patches: &[Patch<T>],
) -> Result<bool, ParseErr> {
    for patch in patches.iter() {
        if (patch.module.is_none() || patch.module == Some(ctx.module()))
            && (patch.match_ident)(get_ident(parsed))
            && (patch.patch)(ctx, parsed)?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
