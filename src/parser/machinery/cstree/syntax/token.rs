use std::{
    fmt,
    hash::{Hash, Hasher},
    iter,
    sync::Arc as StdArc,
};

use lasso::Resolver;
use text_size::{TextRange, TextSize};

use super::{
    super::{interning::Key, Direction, GreenNode, GreenToken, Language, SyntaxKind},
    *,
};

/// Syntax tree token.
#[derive(Debug)]
pub struct SyntaxToken<L: Language, D: 'static = ()> {
    parent: SyntaxNode<L, D>,
    index: u32,
    offset: TextSize,
}

impl<L: Language, D> Clone for SyntaxToken<L, D> {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            index: self.index,
            offset: self.offset,
        }
    }
}

impl<L: Language, D> Hash for SyntaxToken<L, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.index.hash(state);
        self.offset.hash(state);
    }
}

impl<L: Language, D> PartialEq for SyntaxToken<L, D> {
    fn eq(&self, other: &SyntaxToken<L, D>) -> bool {
        self.parent == other.parent && self.index == other.index && self.offset == other.offset
    }
}

impl<L: Language, D> Eq for SyntaxToken<L, D> {}

impl<L: Language, D> SyntaxToken<L, D> {
    /// Writes this token's [`Debug`](fmt::Debug) representation into the given
    /// `target`.
    pub fn write_debug<R>(&self, resolver: &R, target: &mut impl fmt::Write) -> fmt::Result
    where
        R: Resolver + ?Sized,
    {
        write!(target, "{:?}@{:?}", self.kind(), self.text_range())?;
        let text = self.resolve_text(resolver);
        if text.len() < 25 {
            return write!(target, " {:?}", text);
        }

        for idx in 21..25 {
            if text.is_char_boundary(idx) {
                let text = format!("{} ...", &text[..idx]);
                return write!(target, " {:?}", text);
            }
        }
        unreachable!()
    }

    /// Returns this token's [`Debug`](fmt::Debug) representation as a string.
    ///
    /// To avoid allocating for every token, see
    /// [`write_debug`](SyntaxToken::write_debug).
    pub fn debug<R>(&self, resolver: &R) -> String
    where
        R: Resolver + ?Sized,
    {
        // NOTE: `fmt::Write` methods on `String` never fail
        let mut res = String::new();
        self.write_debug(resolver, &mut res).unwrap();
        res
    }

    /// Writes this token's [`Display`](fmt::Display) representation into the
    /// given `target`.
    pub fn write_display<R>(&self, resolver: &R, target: &mut impl fmt::Write) -> fmt::Result
    where
        R: Resolver + ?Sized,
    {
        write!(target, "{}", self.resolve_text(resolver))
    }

    /// Returns this token's [`Display`](fmt::Display) representation as a
    /// string.
    ///
    /// To avoid allocating for every token, see
    /// [`write_display`](SyntaxToken::write_display).
    pub fn display<R>(&self, resolver: &R) -> String
    where
        R: Resolver + ?Sized,
    {
        self.resolve_text(resolver).to_string()
    }

    /// If there is a resolver associated with this tree, returns it.
    pub fn resolver(&self) -> Option<&StdArc<dyn Resolver>> {
        self.parent.resolver()
    }

    /// Turns this token into a [`ResolvedToken`], but only if there is a
    /// resolver associated with this tree.
    pub fn try_resolved(&self) -> Option<&ResolvedToken<L, D>> {
        // safety: we only coerce if `resolver` exists
        self.resolver()
            .map(|_| unsafe { ResolvedToken::coerce_ref(self) })
    }

    /// Turns this token into a [`ResolvedToken`].
    /// # Panics
    /// If there is no resolver associated with this tree.
    pub fn resolved(&self) -> &ResolvedToken<L, D> {
        self.try_resolved()
            .expect("tried to resolve a node without resolver")
    }
}

impl<L: Language, D> SyntaxToken<L, D> {
    pub(super) fn new(
        parent: &SyntaxNode<L, D>,
        index: u32,
        offset: TextSize,
    ) -> SyntaxToken<L, D> {
        Self {
            parent: parent.clone_uncounted(),
            index,
            offset,
        }
    }

    /// Returns a green tree, equal to the green tree this token
    /// belongs two, except with this token substitute. The complexity
    /// of operation is proportional to the depth of the tree
    pub fn replace_with(&self, replacement: GreenToken) -> GreenNode {
        assert_eq!(self.syntax_kind(), replacement.kind());
        let mut replacement = Some(replacement);
        let parent = self.parent();
        let me = self.index;

        let children = parent.green().children().enumerate().map(|(i, child)| {
            if i as u32 == me {
                replacement.take().unwrap().into()
            } else {
                child.cloned()
            }
        });
        let new_parent = GreenNode::new(parent.syntax_kind(), children);
        parent.replace_with(new_parent)
    }

    /// The internal representation of the kind of this token.
    pub fn syntax_kind(&self) -> SyntaxKind {
        self.green().kind()
    }

    /// The kind of this token in terms of your language.
    pub fn kind(&self) -> L::Kind {
        L::kind_from_raw(self.syntax_kind())
    }

    /// The range this token covers in the source text, in bytes.
    pub fn text_range(&self) -> TextRange {
        TextRange::at(self.offset, self.green().text_len())
    }

    /// Uses the provided resolver to return the source text of this token.
    pub fn resolve_text<'i, I>(&self, resolver: &'i I) -> &'i str
    where
        I: Resolver + ?Sized,
    {
        self.green().text(resolver)
    }

    /// Returns `true` if `self` and `other` represent equal source text.
    ///
    /// This method is different from the `PartialEq` and `Eq` implementations
    /// in that it compares the text and not the token position.
    /// It is more efficient than comparing the result of
    /// [`resolve_text`](SyntaxToken::resolve_text) because it compares the
    /// tokens' interned [`text_key`s](SyntaxToken::text_key).
    /// Therefore, it also does not require a [`Resolver`].
    /// **Note** that the result of the comparison may be wrong when comparing
    /// two tokens from different trees that use different interners.
    pub fn text_eq(&self, other: &Self) -> bool {
        self.text_key() == other.text_key()
    }

    /// Returns the interned key of text covered by this token.
    /// This key may be used for comparisons with other keys of strings interned
    /// by the same interner.
    ///
    /// See also [`resolve_text`](SyntaxToken::resolve_text) and
    /// [`text_eq`](SyntaxToken::text_eq).
    pub fn text_key(&self) -> Key {
        self.green().text_key()
    }

    /// Returns the unterlying green tree token of this token.
    pub fn green(&self) -> &GreenToken {
        self.parent
            .green()
            .children()
            .nth(self.index as usize)
            .unwrap()
            .as_token()
            .unwrap()
    }

    /// The parent node of this token.
    pub fn parent(&self) -> &SyntaxNode<L, D> {
        &self.parent
    }

    /// Returns an iterator along the chain of parents of this token.
    pub fn ancestors(&self) -> impl Iterator<Item = &SyntaxNode<L, D>> {
        self.parent().ancestors()
    }

    /// The tree element to the right of this one, i.e. the next child of this
    /// token's parent after this token.
    pub fn next_sibling_or_token(&self) -> Option<SyntaxElementRef<'_, L, D>> {
        self.parent()
            .next_child_or_token_after(self.index as usize, self.text_range().end())
    }

    /// The tree element to the left of this one, i.e. the previous child of
    /// this token's parent after this token.
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElementRef<'_, L, D>> {
        self.parent()
            .prev_child_or_token_before(self.index as usize, self.text_range().start())
    }

    /// Returns an iterator over all siblings of this token in the given
    /// `direction`, i.e. all of this token's parent's children from this
    /// token on to the left or the right. The first item in the iterator
    /// will always be this token.
    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElementRef<'_, L, D>> {
        let me: SyntaxElementRef<'_, L, D> = self.into();
        iter::successors(Some(me), move |el| match direction {
            Direction::Next => el.next_sibling_or_token(),
            Direction::Prev => el.prev_sibling_or_token(),
        })
    }

    /// Returns the next token in the tree.
    /// This is not necessary a direct sibling of this token, but will always be
    /// further right in the tree.
    pub fn next_token(&self) -> Option<&SyntaxToken<L, D>> {
        match self.next_sibling_or_token() {
            Some(element) => element.first_token(),
            None => self
                .parent()
                .ancestors()
                .find_map(|it| it.next_sibling_or_token())
                .and_then(|element| element.first_token()),
        }
    }

    /// Returns the previous token in the tree.
    /// This is not necessary a direct sibling of this token, but will always be
    /// further left in the tree.
    pub fn prev_token(&self) -> Option<&SyntaxToken<L, D>> {
        match self.prev_sibling_or_token() {
            Some(element) => element.last_token(),
            None => self
                .parent()
                .ancestors()
                .find_map(|it| it.prev_sibling_or_token())
                .and_then(|element| element.last_token()),
        }
    }
}
