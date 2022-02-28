use std::{
    collections::{hash_map::RandomState, HashMap},
    convert::TryFrom,
    hash::{BuildHasher, Hash, Hasher},
};

use text_size::TextSize;

use super::{
    super::{
        green::{interner::TokenInterner, GreenElement, GreenNode, GreenToken, SyntaxKind},
        interning::Interner,
        NodeOrToken,
    },
    node::GreenNodeHead,
    token::GreenTokenData,
};

/// If `node.children() <= CHILDREN_CACHE_THRESHOLD`, we will not create
/// a new [`GreenNode`], but instead lookup in the cache if this node is
/// already present. If so we use the one in the cache, otherwise we insert
/// this node into the cache.
const CHILDREN_CACHE_THRESHOLD: usize = 3;

/// A `NodeCache` deduplicates identical tokens and small nodes during tree
/// construction. You can re-use the same cache for multiple similar trees with
/// [`GreenNodeBuilder::with_cache`].
#[derive(Debug)]
pub struct NodeCache<'i, I = TokenInterner> {
    nodes: HashMap<GreenNodeHead, GreenNode>,
    tokens: HashMap<GreenTokenData, GreenToken>,
    interner: MaybeOwned<'i, I>,
}

impl NodeCache<'static> {
    /// Constructs a new, empty cache.
    ///
    /// By default, this will also create a default interner to deduplicate
    /// source text (strings) across tokens. To re-use an existing interner,
    /// see [`with_interner`](NodeCache::with_interner).
    pub fn new() -> Self {
        Self {
            nodes: HashMap::default(),
            tokens: HashMap::default(),
            interner: MaybeOwned::Owned(TokenInterner::new()),
        }
    }
}

impl Default for NodeCache<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'i, I> NodeCache<'i, I>
where
    I: Interner,
{
    /// Constructs a new, empty cache that will use the given interner to
    /// deduplicate source text (strings) across tokens.
    pub fn with_interner(interner: &'i mut I) -> Self {
        Self {
            nodes: HashMap::default(),
            tokens: HashMap::default(),
            interner: MaybeOwned::Borrowed(interner),
        }
    }

    /// Constructs a new, empty cache that will use the given interner to
    /// deduplicate source text (strings) across tokens.
    pub fn from_interner(interner: I) -> Self {
        Self {
            nodes: HashMap::default(),
            tokens: HashMap::default(),
            interner: MaybeOwned::Owned(interner),
        }
    }

    /// Get a reference to the interner used to deduplicate source text
    /// (strings).
    ///
    /// See also [`interner_mut`](NodeCache::interner_mut).
    pub fn interner(&self) -> &I {
        &*self.interner
    }

    /// Get a mutable reference to the interner used to deduplicate source text
    /// (strings).
    pub fn interner_mut(&mut self) -> &mut I {
        &mut *self.interner
    }

    /// If this node cache was constructed with [`new`](NodeCache::new) or
    /// [`from_interner`](NodeCache::from_interner), returns the interner used
    /// to deduplicate source text (strings) to allow resolving tree tokens
    /// back to text and re-using the interner to build additonal trees.
    pub fn into_interner(self) -> Option<I> {
        self.interner.into_owned()
    }

    fn node(&mut self, kind: SyntaxKind, children: &[GreenElement]) -> GreenNode {
        let mut hasher = RandomState::default().build_hasher();
        let mut text_len: TextSize = 0.into();
        for child in children {
            text_len += child.text_len();
            child.hash(&mut hasher);
        }
        let child_hash = hasher.finish() as u32;

        // Green nodes are fully immutable, so it's ok to deduplicate them.
        // This is the same optimization that Roslyn does
        // https://github.com/KirillOsenkov/Bliki/wiki/Roslyn-Immutable-Trees
        //
        // For example, all `#[inline]` in this file share the same green node!
        // For `libsyntax/parse/parser.rs`, measurements show that deduping saves
        // 17% of the memory for green nodes!
        if children.len() <= CHILDREN_CACHE_THRESHOLD {
            self.get_cached_node(kind, children, text_len, child_hash)
        } else {
            GreenNode::new_with_len_and_hash(kind, children.iter().cloned(), text_len, child_hash)
        }
    }

    /// Creates a [`GreenNode`] by looking inside the cache or inserting
    /// a new node into the cache if it's a cache miss.
    fn get_cached_node(
        &mut self,
        kind: SyntaxKind,
        children: &[GreenElement],
        text_len: TextSize,
        child_hash: u32,
    ) -> GreenNode {
        let head = GreenNodeHead {
            kind,
            text_len,
            child_hash,
        };
        self.nodes
            .entry(head)
            .or_insert_with_key(|head| {
                GreenNode::from_head_and_children(head.clone(), children.iter().cloned())
            })
            .clone()
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) -> GreenToken {
        let text_len = TextSize::try_from(text.len()).unwrap();
        let text = self.interner.get_or_intern(text);
        let data = GreenTokenData {
            kind,
            text,
            text_len,
        };
        self.tokens
            .entry(data)
            .or_insert_with_key(|data| GreenToken::new(*data))
            .clone()
    }
}

#[derive(Debug)]
enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<T> MaybeOwned<'_, T> {
    fn into_owned(self) -> Option<T> {
        match self {
            MaybeOwned::Owned(owned) => Some(owned),
            MaybeOwned::Borrowed(_) => None,
        }
    }
}

impl<T> std::ops::Deref for MaybeOwned<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            MaybeOwned::Owned(it) => it,
            MaybeOwned::Borrowed(it) => *it,
        }
    }
}

impl<T> std::ops::DerefMut for MaybeOwned<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            MaybeOwned::Owned(it) => it,
            MaybeOwned::Borrowed(it) => *it,
        }
    }
}

impl<T: Default> Default for MaybeOwned<'_, T> {
    fn default() -> Self {
        MaybeOwned::Owned(T::default())
    }
}

/// A checkpoint for maybe wrapping a node. See [`GreenNodeBuilder::checkpoint`]
/// for details.
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(usize);

/// A builder for green trees.
/// Construct with [`new`](GreenNodeBuilder::new),
/// [`with_cache`](GreenNodeBuilder::with_cache), or
/// [`from_cache`](GreenNodeBuilder::from_cache). To add tree nodes, start them
/// with [`start_node`](GreenNodeBuilder::start_node), add
/// [`token`](GreenNodeBuilder::token)s and then
/// [`finish_node`](GreenNodeBuilder::finish_node). When the whole tree is
/// constructed, call [`finish`](GreenNodeBuilder::finish) to obtain the root.
#[derive(Debug)]
pub struct GreenNodeBuilder<'cache, 'interner, I = TokenInterner> {
    cache: MaybeOwned<'cache, NodeCache<'interner, I>>,
    parents: Vec<(SyntaxKind, usize)>,
    children: Vec<GreenElement>,
}

impl GreenNodeBuilder<'static, 'static> {
    /// Creates new builder with an empty [`NodeCache`].
    pub fn new() -> Self {
        Self {
            cache: MaybeOwned::Owned(NodeCache::new()),
            parents: Vec::with_capacity(8),
            children: Vec::with_capacity(8),
        }
    }
}

impl Default for GreenNodeBuilder<'static, 'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'cache, 'interner, I> GreenNodeBuilder<'cache, 'interner, I>
where
    I: Interner,
{
    /// Reusing a [`NodeCache`] between multiple builders saves memory, as it
    /// allows to structurally share underlying trees.
    pub fn with_cache(cache: &'cache mut NodeCache<'interner, I>) -> Self {
        Self {
            cache: MaybeOwned::Borrowed(cache),
            parents: Vec::with_capacity(8),
            children: Vec::with_capacity(8),
        }
    }

    /// Reusing a [`NodeCache`] between multiple builders saves memory, as it
    /// allows to structurally share underlying trees.
    /// The `cache` given will be returned on
    /// [`finish`](GreenNodeBuilder::finish).
    pub fn from_cache(cache: NodeCache<'interner, I>) -> Self {
        Self {
            cache: MaybeOwned::Owned(cache),
            parents: Vec::with_capacity(8),
            children: Vec::with_capacity(8),
        }
    }

    /// Shortcut to construct a builder that uses an existing interner.
    ///
    /// This is equivalent to using [`from_cache`](GreenNodeBuilder::from_cache)
    /// with a node cache obtained from [`NodeCache::with_interner`].
    pub fn with_interner(interner: &'interner mut I) -> Self {
        let cache = NodeCache::with_interner(interner);
        Self::from_cache(cache)
    }

    /// Shortcut to construct a builder that uses an existing interner.
    ///
    /// This is equivalent to using [`from_cache`](GreenNodeBuilder::from_cache)
    /// with a node cache obtained from [`NodeCache::from_interner`].
    pub fn from_interner(interner: I) -> Self {
        let cache = NodeCache::from_interner(interner);
        Self::from_cache(cache)
    }

    /// Get a reference to the interner used to deduplicate source text
    /// (strings).
    ///
    /// This is the same interner as used by the underlying [`NodeCache`].
    /// See also [`interner_mut`](GreenNodeBuilder::interner_mut).
    pub fn interner(&self) -> &I {
        &*self.cache.interner
    }

    /// Get a mutable reference to the interner used to deduplicate source text
    /// (strings).
    ///
    /// This is the same interner as used by the underlying [`NodeCache`].
    pub fn interner_mut(&mut self) -> &mut I {
        &mut *self.cache.interner
    }

    /// Add new token to the current branch.
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let token = self.cache.token(kind, text);
        self.children.push(token.into());
    }

    /// Start new node of the given `kind` and make it current.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        let len = self.children.len();
        self.parents.push((kind, len));
    }

    /// Finish the current branch and restore the previous branch as current.
    pub fn finish_node(&mut self) {
        let (kind, first_child) = self.parents.pop().unwrap();
        let node = self.cache.node(kind, &self.children[first_child..]);
        self.children.truncate(first_child);
        self.children.push(node.into());
    }

    /// Prepare for maybe wrapping the next node with a surrounding node.
    ///
    /// The way wrapping works is that you first get a checkpoint, then you add
    /// nodes and tokens as normal, and then you *maybe* call
    /// [`start_node_at`](GreenNodeBuilder::start_node_at).
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.children.len())
    }

    /// Wrap the previous branch marked by
    /// [`checkpoint`](GreenNodeBuilder::checkpoint) in a new branch and
    /// make it current.
    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        let Checkpoint(checkpoint) = checkpoint;
        assert!(
            checkpoint <= self.children.len(),
            "checkpoint no longer valid, was finish_node called early?"
        );

        if let Some(&(_, first_child)) = self.parents.last() {
            assert!(
                checkpoint >= first_child,
                "checkpoint no longer valid, was an unmatched start_node_at called?"
            );
        }

        self.parents.push((kind, checkpoint));
    }

    /// Complete building the tree.
    ///
    /// Make sure that calls to [`start_node`](GreenNodeBuilder::start_node) /
    /// [`start_node_at`](GreenNodeBuilder::start_node_at) and
    /// [`finish_node`](GreenNodeBuilder::finish_node) are balanced, i.e. that
    /// every started node has been completed!
    ///
    /// If this builder was constructed with [`new`](GreenNodeBuilder::new) or
    /// [`from_cache`](GreenNodeBuilder::from_cache), this method returns the
    /// cache used to deduplicate tree nodes  as its second return value to
    /// allow re-using the cache or extracting the underlying string
    ///  [`Interner`]. See also [`NodeCache::into_interner`].
    pub fn finish(mut self) -> (GreenNode, Option<NodeCache<'interner, I>>) {
        assert_eq!(self.children.len(), 1);
        let cache = self.cache.into_owned();
        match self.children.pop().unwrap() {
            NodeOrToken::Node(node) => (node, cache),
            NodeOrToken::Token(_) =>
                panic!("called `finish` on a `GreenNodeBuilder` which only contained a token"),
        }
    }
}
