use super::{GetSpan, Parse, ParseErr, ParseRawRes, ParseRev, Span, WsAndComments};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct DocComment {
    pub span: Span,
}

impl GetSpan for DocComment {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl DocComment {
    pub fn try_parse_combine_postfix(
        pre: Option<DocComment>,
        input: &mut Span,
    ) -> Result<Option<DocComment>, ParseErr> {
        if let Some(post) = DocCommentPost::try_parse(input)? {
            if pre.is_some() {
                Err(ParseErr::new(
                    post.span,
                    "item has both prefix and postfix documentation comment",
                ))
            } else {
                Ok(Some(post.into()))
            }
        } else {
            Ok(pre)
        }
    }

    pub fn try_parse_rev_combine_postfix(
        pre: Option<DocComment>,
        input: &mut Span,
    ) -> Result<Option<DocComment>, ParseErr> {
        if let Some(post) = DocCommentPost::try_parse_rev(input)? {
            if pre.is_some() {
                Err(ParseErr::new(
                    post.span,
                    "item has both prefix and postfix documentation comment",
                ))
            } else {
                Ok(Some(post.into()))
            }
        } else {
            Ok(pre)
        }
    }
}

impl Parse for DocComment {
    fn desc() -> Cow<'static, str> {
        "documentation comment".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_start()?;
        if rest.starts_with("/**") && !rest.starts_with("/**<") {
            if let Some(end) = rest.as_bytes().windows(2).position(|b| b == b"*/") {
                let span = rest.slice(3..end).trim();
                // strip whitespace after the doc comment, up to max one newline
                let mut rest = rest.slice(end + 2..);
                let mut got_cr = false;
                for (i, ch) in rest.char_indices() {
                    if ch == '\n' {
                        if got_cr {
                            rest = rest.slice(i..);
                            return Ok((rest, Some(Self { span })));
                        }
                        got_cr = true;
                    } else if !ch.is_ascii_whitespace() {
                        rest = rest.slice(i..);
                        return Ok((rest, Some(Self { span })));
                    }
                }
                Ok((rest.end(), Some(Self { span })))
            } else {
                Err(ParseErr::new(rest.slice(..4), "doc comment with no end"))
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

pub struct DocCommentFile(DocComment);

impl Parse for DocCommentFile {
    fn desc() -> Cow<'static, str> {
        "doc comment for file".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        if let (mut rest, Some(dc)) = DocComment::try_parse_raw(input)? {
            if WsAndComments::try_parse(&mut rest)?.is_some() {
                // empty line after doc comment, so it's not attached to anything
                return Ok((rest, Some(Self(dc))));
            }
        }
        Ok((input.clone(), None))
    }
}

impl From<DocCommentFile> for DocComment {
    fn from(value: DocCommentFile) -> Self {
        value.0
    }
}

pub struct DocCommentPost {
    span: Span,
}

impl Parse for DocCommentPost {
    fn desc() -> Cow<'static, str> {
        "documentation comment (postfix)".into()
    }

    fn try_parse_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_start()?;
        if rest.starts_with("/**<") {
            if let Some(end) = rest.as_bytes().windows(2).position(|b| b == b"*/") {
                let (span, rest) = rest.split_at(end + 2);
                Ok((
                    rest,
                    Some(Self {
                        span: span.slice(4..span.len() - 2).trim(),
                    }),
                ))
            } else {
                todo!()
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl ParseRev for DocCommentPost {
    fn try_parse_rev_raw(input: &Span) -> ParseRawRes<Option<Self>> {
        let rest = input.trim_wsc_end()?;
        if rest.ends_with("*/") {
            if let Some(start) = rest.as_bytes().windows(4).rev().position(|b| b == b"/**<") {
                let (rest, span) = rest.split_at(rest.len() - start - 4);
                Ok((
                    rest,
                    Some(Self {
                        span: span.slice(4..span.len() - 2).trim(),
                    }),
                ))
            } else {
                todo!()
            }
        } else {
            Ok((input.clone(), None))
        }
    }
}

impl From<DocCommentPost> for DocComment {
    fn from(value: DocCommentPost) -> Self {
        Self { span: value.span }
    }
}
