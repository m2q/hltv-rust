/*!
Extensions to make the [`tl`] crate more ergonomic.

This module contains functionality that makes it easier to traverse
DOM elements and NodeHandles, as well as extract information (without
the overhead of the NodeHandle -> Node abstraction).
*/

use crate::Error;
use std::{borrow::Cow, str::FromStr};
use tl::*;

pub trait HTMLTagExtension {
    /// Finds an attribute in a given html tag. Returns `Some(x)` if attribute `x`
    /// could be found and parsed correctly. Returns a [`Error::ConversionError`]
    /// if the attribute string couldn't be parsed to `T`.
    fn get_attr<T>(&self, attr: &str) -> Result<Option<T>, Error>
    where
        T: FromStr;

    /// Finds an attribute in a given html tag. Returns a normal String.
    fn get_attr_str(&self, attr: &str) -> Option<String>;
}

impl<'a> HTMLTagExtension for HTMLTag<'a> {
    fn get_attr<T>(&self, attr: &str) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        let s = self.get_attr_str(attr);
        if s.is_none(){
            return Ok(None);
        }
        match s.unwrap().parse::<T>() {
            Ok(t) => Ok(Some(t)),
            Err(_) => Err(Error::ParseError),
        }
    }

    fn get_attr_str(&self, attr: &str) -> Option<String> {
        let result = self.attributes().get(attr).flatten()?;
        Some(result.as_utf8_str().to_string())
    }
}

pub trait NodeHandleExtension {
    fn inner_text<'b, 'p: 'b>(&'b self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>>;
}

impl<'a> NodeHandleExtension for NodeHandle {
    fn inner_text<'b, 'p: 'b>(&'b self, parser: &'p tl::Parser<'b>) -> Option<Cow<'b, str>> {
        let node = self.get(parser)?;
        Some(node.inner_text(parser))
    }
}

pub trait VDomExtension<'a> {
    /// Return the first node in the subtree of `h` that has the given
    /// class string.
    fn select_first(&'a self, h: NodeHandle, class: &str) -> Option<NodeHandle>;
    /// Return all nodes in the subtree of `h` that have the given
    /// class string.
    fn select_nodes(&'a self, h: NodeHandle, class: &str) -> Vec<NodeHandle>;
}

impl<'a> VDomExtension<'a> for VDom<'a> {
    fn select_first(&'a self, h: NodeHandle, class: &str) -> Option<NodeHandle> {
        dfs_first(h, self, class)
    }
    fn select_nodes(&'a self, h: NodeHandle, class: &str) -> Vec<NodeHandle> {
        let mut result = Vec::<NodeHandle>::new();
        dfs(h, self, class, &mut result);
        result
    }
}

/// Populate a vector with children of the given NodeHandle that
/// match the class string. Traverses the subtree via depth-first search.
fn dfs<'a>(h: NodeHandle, dom: &'a VDom<'a>, class: &str, result: &mut Vec<NodeHandle>) {
    // break condition
    let tag = h.get(dom.parser()).unwrap().as_tag();
    if tag.is_none() {
        return;
    }

    if tag.unwrap().attributes().is_class_member(class) {
        result.push(h);
    }

    // return None when no children
    let children = h.get(dom.parser()).unwrap().children();
    if children.is_none() {
        return;
    }

    // otherwise, iterate over all children
    for c in children.unwrap() {
        dfs(c, dom, class, result);
    }
}

fn dfs_first<'a>(h: NodeHandle, dom: &'a VDom<'a>, class: &str) -> Option<NodeHandle> {
    // break condition
    let tag = h.get(dom.parser()).unwrap().as_tag()?;
    if tag.attributes().is_class_member(class) {
        return Some(h);
    }
    // return None when no children
    let children = h.get(dom.parser()).unwrap().children()?;

    // otherwise, iterate over all children
    for c in children {
        if let Some(x) = dfs_first(c, dom, class) {
            return Some(x);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn dfs_test() {
        let input = include_str!("./testdata/dfs.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let nodes = dom.select_nodes(dom.children()[0], "abc");
        assert_eq!(nodes.len(), 2);
        assert_eq!(
            nodes[0].get(dom.parser()).unwrap().inner_text(dom.parser()),
            "dist1ll"
        );
        let node = dom.select_first(dom.children()[0], "abc").unwrap();
        assert_eq!(node.inner_text(dom.parser()).unwrap(), "dist1ll");

        let nodes = dom.select_nodes(dom.children()[1], "abc");
        assert_eq!(nodes.len(), 1);
    }
}
