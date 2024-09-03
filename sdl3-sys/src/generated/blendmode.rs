#![allow(non_camel_case_types, non_upper_case_globals, clippy::approx_constant, clippy::double_parens)]

//! # CategoryBlendmode
//!
//! Blend modes decide how two colors will mix together. There are both
//! standard modes for basic needs and a means to create custom modes,
//! dictating what sort of math to do what on what color components.

use super::stdinc::*;

/// A set of blend modes used in drawing operations.
///
/// These predefined blend modes are supported everywhere.
///
/// Additional values may be obtained from SDL_ComposeCustomBlendMode.
///
/// \since This datatype is available since SDL 3.0.0.
///
/// \sa SDL_ComposeCustomBlendMode
pub type SDL_BlendMode = Uint32;

pub const SDL_BLENDMODE_NONE: ::core::primitive::u32 = 0;

pub const SDL_BLENDMODE_BLEND: ::core::primitive::u32 = 1;

pub const SDL_BLENDMODE_BLEND_PREMULTIPLIED: ::core::primitive::u32 = 16;

pub const SDL_BLENDMODE_ADD: ::core::primitive::u32 = 2;

pub const SDL_BLENDMODE_ADD_PREMULTIPLIED: ::core::primitive::u32 = 32;

pub const SDL_BLENDMODE_MOD: ::core::primitive::u32 = 4;

pub const SDL_BLENDMODE_MUL: ::core::primitive::u32 = 8;

pub const SDL_BLENDMODE_INVALID: ::core::primitive::u32 = 2147483647;

/// The blend operation used when combining source and destination pixel
/// components.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_BLENDOPERATION_ADD`], [`SDL_BLENDOPERATION_SUBTRACT`], [`SDL_BLENDOPERATION_REV_SUBTRACT`], [`SDL_BLENDOPERATION_MINIMUM`], [`SDL_BLENDOPERATION_MAXIMUM`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_BlendOperation(pub ::core::ffi::c_int);
impl SDL_BlendOperation {
    /// dst + src: supported by all renderers
    pub const ADD: Self = Self(0x1);
    /// src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const SUBTRACT: Self = Self(0x2);
    /// dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const REV_SUBTRACT: Self = Self(0x3);
    /// min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const MINIMUM: Self = Self(0x4);
    /// max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
    pub const MAXIMUM: Self = Self(0x5);
}
/// dst + src: supported by all renderers
pub const SDL_BLENDOPERATION_ADD: SDL_BlendOperation = SDL_BlendOperation::ADD;
/// src - dst : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_SUBTRACT: SDL_BlendOperation = SDL_BlendOperation::SUBTRACT;
/// dst - src : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_REV_SUBTRACT: SDL_BlendOperation = SDL_BlendOperation::REV_SUBTRACT;
/// min(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_MINIMUM: SDL_BlendOperation = SDL_BlendOperation::MINIMUM;
/// max(dst, src) : supported by D3D, OpenGL, OpenGLES, and Vulkan
pub const SDL_BLENDOPERATION_MAXIMUM: SDL_BlendOperation = SDL_BlendOperation::MAXIMUM;

/// The normalized factor used to multiply pixel components.
///
/// The blend factors are multiplied with the pixels from a drawing operation
/// (src) and the pixels from the render target (dst) before the blend
/// operation. The comma-separated factors listed above are always applied in
/// the component order red, green, blue, and alpha.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_BLENDFACTOR_ZERO`], [`SDL_BLENDFACTOR_ONE`], [`SDL_BLENDFACTOR_SRC_COLOR`], [`SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR`], [`SDL_BLENDFACTOR_SRC_ALPHA`], [`SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA`], [`SDL_BLENDFACTOR_DST_COLOR`], [`SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR`], [`SDL_BLENDFACTOR_DST_ALPHA`], [`SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_BlendFactor(pub ::core::ffi::c_int);
impl SDL_BlendFactor {
    /// 0, 0, 0, 0
    pub const ZERO: Self = Self(0x1);
    /// 1, 1, 1, 1
    pub const ONE: Self = Self(0x2);
    /// srcR, srcG, srcB, srcA
    pub const SRC_COLOR: Self = Self(0x3);
    /// 1-srcR, 1-srcG, 1-srcB, 1-srcA
    pub const ONE_MINUS_SRC_COLOR: Self = Self(0x4);
    /// srcA, srcA, srcA, srcA
    pub const SRC_ALPHA: Self = Self(0x5);
    /// 1-srcA, 1-srcA, 1-srcA, 1-srcA
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(0x6);
    /// dstR, dstG, dstB, dstA
    pub const DST_COLOR: Self = Self(0x7);
    /// 1-dstR, 1-dstG, 1-dstB, 1-dstA
    pub const ONE_MINUS_DST_COLOR: Self = Self(0x8);
    /// dstA, dstA, dstA, dstA
    pub const DST_ALPHA: Self = Self(0x9);
    /// 1-dstA, 1-dstA, 1-dstA, 1-dstA
    pub const ONE_MINUS_DST_ALPHA: Self = Self(0xa);
}
/// 0, 0, 0, 0
pub const SDL_BLENDFACTOR_ZERO: SDL_BlendFactor = SDL_BlendFactor::ZERO;
/// 1, 1, 1, 1
pub const SDL_BLENDFACTOR_ONE: SDL_BlendFactor = SDL_BlendFactor::ONE;
/// srcR, srcG, srcB, srcA
pub const SDL_BLENDFACTOR_SRC_COLOR: SDL_BlendFactor = SDL_BlendFactor::SRC_COLOR;
/// 1-srcR, 1-srcG, 1-srcB, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_COLOR: SDL_BlendFactor = SDL_BlendFactor::ONE_MINUS_SRC_COLOR;
/// srcA, srcA, srcA, srcA
pub const SDL_BLENDFACTOR_SRC_ALPHA: SDL_BlendFactor = SDL_BlendFactor::SRC_ALPHA;
/// 1-srcA, 1-srcA, 1-srcA, 1-srcA
pub const SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: SDL_BlendFactor = SDL_BlendFactor::ONE_MINUS_SRC_ALPHA;
/// dstR, dstG, dstB, dstA
pub const SDL_BLENDFACTOR_DST_COLOR: SDL_BlendFactor = SDL_BlendFactor::DST_COLOR;
/// 1-dstR, 1-dstG, 1-dstB, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_COLOR: SDL_BlendFactor = SDL_BlendFactor::ONE_MINUS_DST_COLOR;
/// dstA, dstA, dstA, dstA
pub const SDL_BLENDFACTOR_DST_ALPHA: SDL_BlendFactor = SDL_BlendFactor::DST_ALPHA;
/// 1-dstA, 1-dstA, 1-dstA, 1-dstA
pub const SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA: SDL_BlendFactor = SDL_BlendFactor::ONE_MINUS_DST_ALPHA;

extern_sdlcall! {{
    /// Compose a custom blend mode for renderers.
    ///
    /// The functions SDL_SetRenderDrawBlendMode and SDL_SetTextureBlendMode accept
    /// the SDL_BlendMode returned by this function if the renderer supports it.
    ///
    /// A blend mode controls how the pixels from a drawing operation (source) get
    /// combined with the pixels from the render target (destination). First, the
    /// components of the source and destination pixels get multiplied with their
    /// blend factors. Then, the blend operation takes the two products and
    /// calculates the result that will get stored in the render target.
    ///
    /// Expressed in pseudocode, it would look like this:
    ///
    /// ```c
    /// dstRGB = colorOperation(srcRGB * srcColorFactor, dstRGB * dstColorFactor);
    /// dstA = alphaOperation(srcA * srcAlphaFactor, dstA * dstAlphaFactor);
    /// ```
    ///
    /// Where the functions `colorOperation(src, dst)` and `alphaOperation(src,
    /// dst)` can return one of the following:
    ///
    /// - `src + dst`
    /// - `src - dst`
    /// - `dst - src`
    /// - `min(src, dst)`
    /// - `max(src, dst)`
    ///
    /// The red, green, and blue components are always multiplied with the first,
    /// second, and third components of the SDL_BlendFactor, respectively. The
    /// fourth component is not used.
    ///
    /// The alpha component is always multiplied with the fourth component of the
    /// SDL_BlendFactor. The other components are not used in the alpha
    /// calculation.
    ///
    /// Support for these blend modes varies for each renderer. To check if a
    /// specific SDL_BlendMode is supported, create a renderer and pass it to
    /// either SDL_SetRenderDrawBlendMode or SDL_SetTextureBlendMode. They will
    /// return with an error if the blend mode is not supported.
    ///
    /// This list describes the support of custom blend modes for each renderer.
    /// All renderers support the four blend modes listed in the SDL_BlendMode
    /// enumeration.
    ///
    /// - **direct3d**: Supports all operations with all factors. However, some
    ///   factors produce unexpected results with `SDL_BLENDOPERATION_MINIMUM` and
    ///   `SDL_BLENDOPERATION_MAXIMUM`.
    /// - **direct3d11**: Same as Direct3D 9.
    /// - **opengl**: Supports the `SDL_BLENDOPERATION_ADD` operation with all
    ///   factors. OpenGL versions 1.1, 1.2, and 1.3 do not work correctly here.
    /// - **opengles2**: Supports the `SDL_BLENDOPERATION_ADD`,
    ///   `SDL_BLENDOPERATION_SUBTRACT`, `SDL_BLENDOPERATION_REV_SUBTRACT`
    ///   operations with all factors.
    /// - **psp**: No custom blend mode support.
    /// - **software**: No custom blend mode support.
    ///
    /// Some renderers do not provide an alpha component for the default render
    /// target. The `SDL_BLENDFACTOR_DST_ALPHA` and
    /// `SDL_BLENDFACTOR_ONE_MINUS_DST_ALPHA` factors do not have an effect in this
    /// case.
    ///
    /// \param srcColorFactor the SDL_BlendFactor applied to the red, green, and
    ///                       blue components of the source pixels.
    /// \param dstColorFactor the SDL_BlendFactor applied to the red, green, and
    ///                       blue components of the destination pixels.
    /// \param colorOperation the SDL_BlendOperation used to combine the red,
    ///                       green, and blue components of the source and
    ///                       destination pixels.
    /// \param srcAlphaFactor the SDL_BlendFactor applied to the alpha component of
    ///                       the source pixels.
    /// \param dstAlphaFactor the SDL_BlendFactor applied to the alpha component of
    ///                       the destination pixels.
    /// \param alphaOperation the SDL_BlendOperation used to combine the alpha
    ///                       component of the source and destination pixels.
    /// \returns an SDL_BlendMode that represents the chosen factors and
    ///          operations.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetRenderDrawBlendMode
    /// \sa SDL_GetRenderDrawBlendMode
    /// \sa SDL_SetTextureBlendMode
    /// \sa SDL_GetTextureBlendMode
    pub fn SDL_ComposeCustomBlendMode(srcColorFactor: SDL_BlendFactor, dstColorFactor: SDL_BlendFactor, colorOperation: SDL_BlendOperation, srcAlphaFactor: SDL_BlendFactor, dstAlphaFactor: SDL_BlendFactor, alphaOperation: SDL_BlendOperation) -> SDL_BlendMode;
}}
