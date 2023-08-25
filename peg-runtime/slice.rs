use super::{Parse, ParseElem, ParseLiteral, ParseSlice, RuleResult};

impl<T> Parse for [T] {
    type PositionRepr = usize;
    fn start(&self) -> usize {
        0
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.len()
    }

    fn position_repr(&self, pos: usize) -> usize {
        pos
    }
}

impl<'input, T: 'input + Copy> ParseElem<'input> for [T] {
    type Element = T;

    fn parse_elem(&'input self, pos: usize) -> RuleResult<T> {
        match self[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, *c),
            None => RuleResult::Failed,
        }
    }
}

impl<'input, T: 'input + Clone> ParseElem<'input> for [T] {
    type Element = T;

    fn parse_elem(&'input self, pos: usize) -> RuleResult<T> {
        match self[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, c.clone()),
            None => RuleResult::Failed,
        }
    }
}

impl ParseLiteral for [u8] {
    fn parse_string_literal(&self, pos: usize, literal: &str) -> RuleResult<()> {
        let l = literal.len();
        if self.len() >= pos + l && &self[pos..pos + l] == literal.as_bytes() {
            RuleResult::Matched(pos + l, ())
        } else {
            RuleResult::Failed
        }
    }
}

impl<'input, T: 'input> ParseSlice<'input> for [T] {
    type Slice = &'input [T];
    fn parse_slice(&'input self, p1: usize, p2: usize) -> &'input [T] {
        &self[p1..p2]
    }
}

pub struct SliceByRef<'input, T>(pub &'input [T]);

impl<'input, T> From<&'input [T]> for SliceByRef<'input, T> {
    fn from(value: &'input [T]) -> Self {
        Self(value)
    }
}

impl<'input, T> Parse for SliceByRef<'input, T> {
    type PositionRepr = usize;
    fn start(&self) -> usize {
        0
    }

    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.0.len()
    }

    fn position_repr(&self, pos: usize) -> usize {
        pos
    }
}

impl<'input, T: 'input> ParseElem<'input> for SliceByRef<'input, T> {
    type Element = &'input T;

    fn parse_elem(&'input self, pos: usize) -> RuleResult<&'input T> {
        match self.0[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, c),
            None => RuleResult::Failed,
        }
    }
}

impl<'input, T: 'input> ParseSlice<'input> for SliceByRef<'input, T> {
    type Slice = &'input [T];
    fn parse_slice(&'input self, p1: usize, p2: usize) -> &'input [T] {
        &self.0[p1..p2]
    }
}
