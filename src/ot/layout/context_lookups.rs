//! Contextual and chaining contextual lookups.

use std::convert::TryFrom;

use ttf_parser::parser::{FromData, LazyArray16, Offset, Offset16, Offsets16, Stream};
use ttf_parser::GlyphId;

use super::apply::{ApplyContext, WouldApplyContext};
use super::common::{ClassDef, Coverage};
use super::matching::{
    match_backtrack, match_class, match_coverage, match_glyph, match_input, match_lookahead,
    would_match_input, MatchFunc, Matched,
};
use super::MAX_CONTEXT_LENGTH;

#[derive(Clone, Copy, Debug)]
struct LookupRecord {
    sequence_index: u16,
    lookup_list_index: u16,
}

impl FromData for LookupRecord {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            sequence_index: s.read::<u16>()?,
            lookup_list_index: s.read::<u16>()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum ContextLookup<'a> {
    Format1 {
        coverage: Coverage<'a>,
        sets: Offsets16<'a, Offset16>,
    },
    Format2 {
        coverage: Coverage<'a>,
        classes: ClassDef<'a>,
        sets: Offsets16<'a, Offset16>,
    },
    Format3 {
        data: &'a [u8],
        coverage: Coverage<'a>,
        coverages: LazyArray16<'a, u16>,
        lookups: LazyArray16<'a, LookupRecord>,
    },
}

impl<'a> ContextLookup<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format: u16 = s.read()?;
        Some(match format {
            1 => {
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let count = s.read::<u16>()?;
                let sets = s.read_offsets16(count, data)?;
                Self::Format1 { coverage, sets }
            }
            2 => {
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let classes = ClassDef::parse(s.read_offset16_data()?)?;
                let count = s.read::<u16>()?;
                let sets = s.read_offsets16(count, data)?;
                Self::Format2 { coverage, classes, sets }
            }
            3 => {
                let input_count = s.read::<u16>()?;
                let lookup_count = s.read::<u16>()?;
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let coverages = s.read_array16(input_count.checked_sub(1)?)?;
                let lookups = s.read_array16(lookup_count)?;
                Self::Format3 { data, coverage, coverages, lookups }
            }
            _ => return None,
        })
    }

    fn coverage(&self) -> &Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => coverage,
            Self::Format2 { coverage, .. } => coverage,
            Self::Format3 { coverage, .. } => coverage,
        }
    }

    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = GlyphId(u16::try_from(ctx.glyph(0)).unwrap());
        match *self {
            Self::Format1 { coverage, sets } => {
                coverage.get(glyph_id)
                    .and_then(|index| sets.slice(index))
                    .and_then(RuleSet::parse)
                    .map(|set| set.would_apply(ctx, &match_glyph))
                    .unwrap_or(false)
            }
            Self::Format2 { classes: class_def, sets, .. } => {
                let class = class_def.get(glyph_id);
                sets.get(class.0).map_or(false, |offset| !offset.is_null())
                    && sets.slice(class.0)
                        .and_then(RuleSet::parse)
                        .map(|set| set.would_apply(ctx, &match_class(class_def)))
                        .unwrap_or(false)
            }
            Self::Format3 { data, coverages, .. } => {
                would_apply_context(ctx, coverages, &match_coverage(data))
            }
        }
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let glyph_id = GlyphId(u16::try_from(ctx.buffer().cur(0).codepoint).unwrap());
        match *self {
            Self::Format1 { coverage, sets } => {
                let index = coverage.get(glyph_id)?;
                let set = RuleSet::parse(sets.slice(index)?)?;
                set.apply(ctx, &match_glyph)
            }
            Self::Format2 { coverage, classes, sets } => {
                coverage.get(glyph_id)?;
                let class = classes.get(glyph_id);
                let offset = sets.get(class.0)?;
                if !offset.is_null() {
                    let set = RuleSet::parse(sets.slice(class.0)?)?;
                    set.apply(ctx, &match_class(classes))
                } else {
                    None
                }
            }
            Self::Format3 { data, coverage, coverages, lookups } => {
                coverage.get(glyph_id)?;
                apply_context(ctx, coverages, &match_coverage(data), lookups)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RuleSet<'a> {
    rules: Offsets16<'a, Offset16>,
}

impl<'a> RuleSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let rules = s.read_offsets16(count, data)?;
        Some(Self { rules })
    }

    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &MatchFunc) -> bool {
        self.rules
            .into_iter()
            .filter_map(|data| Rule::parse(data))
            .any(|rules| rules.would_apply(ctx, match_func))
    }

    fn apply(&self, ctx: &mut ApplyContext, match_func: &MatchFunc) -> Option<()> {
        for data in self.rules {
            if let Some(rule) = Rule::parse(data) {
                if rule.apply(ctx, match_func).is_some() {
                    return Some(());
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct Rule<'a> {
    input: LazyArray16<'a, u16>,
    lookups: LazyArray16<'a, LookupRecord>,
}

impl<'a> Rule<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let input_count = s.read::<u16>()?;
        let lookup_count = s.read::<u16>()?;
        let input = s.read_array16(input_count.checked_sub(1)?)?;
        let lookups = s.read_array16(lookup_count)?;
        Some(Self { input, lookups })
    }

    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &MatchFunc) -> bool {
        would_apply_context(ctx, self.input, match_func)
    }

    fn apply(&self, ctx: &mut ApplyContext, match_func: &MatchFunc) -> Option<()> {
        apply_context(ctx, self.input, match_func, self.lookups)
    }
}

#[derive(Clone, Copy, Debug)]
enum ChainContextLookup<'a> {
    Format1 {
        coverage: Coverage<'a>,
        sets: Offsets16<'a, Offset16>,
    },
    Format2 {
        coverage: Coverage<'a>,
        backtrack_classes: ClassDef<'a>,
        input_classes: ClassDef<'a>,
        lookahead_classes: ClassDef<'a>,
        sets: Offsets16<'a, Offset16>,
    },
    Format3 {
        data: &'a [u8],
        coverage: Coverage<'a>,
        backtrack_coverages: LazyArray16<'a, u16>,
        input_coverages: LazyArray16<'a, u16>,
        lookahead_coverages: LazyArray16<'a, u16>,
        lookups: LazyArray16<'a, LookupRecord>,
    },
}

impl<'a> ChainContextLookup<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format: u16 = s.read()?;
        Some(match format {
            1 => {
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let count = s.read::<u16>()?;
                let sets = s.read_offsets16(count, data)?;
                Self::Format1 { coverage, sets }
            }
            2 => {
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let backtrack_classes = ClassDef::parse(s.read_offset16_data()?)?;
                let input_classes = ClassDef::parse(s.read_offset16_data()?)?;
                let lookahead_classes = ClassDef::parse(s.read_offset16_data()?)?;
                let count = s.read::<u16>()?;
                let sets = s.read_offsets16(count, data)?;
                Self::Format2 {
                    coverage,
                    backtrack_classes,
                    input_classes,
                    lookahead_classes,
                    sets,
                }
            }
            3 => {
                let backtrack_count = s.read::<u16>()?;
                let backtrack_coverages = s.read_array16(backtrack_count)?;
                let input_count = s.read::<u16>()?;
                let coverage = Coverage::parse(s.read_offset16_data()?)?;
                let input_coverages = s.read_array16(input_count.checked_sub(1)?)?;
                let lookahead_count = s.read::<u16>()?;
                let lookahead_coverages = s.read_array16(lookahead_count)?;
                let lookup_count = s.read::<u16>()?;
                let lookups = s.read_array16(lookup_count)?;
                Self::Format3 {
                    data,
                    coverage,
                    backtrack_coverages,
                    input_coverages,
                    lookahead_coverages,
                    lookups,
                }
            }
            _ => return None,
        })
    }

    fn coverage(&self) -> &Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => coverage,
            Self::Format2 { coverage, .. } => coverage,
            Self::Format3 { coverage, .. } => coverage,
        }
    }

    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let glyph_id = GlyphId(u16::try_from(ctx.glyph(0)).unwrap());
        match *self {
            Self::Format1 { coverage, sets } => {
                coverage.get(glyph_id)
                    .and_then(|index| sets.slice(index))
                    .and_then(ChainRuleSet::parse)
                    .map(|set| set.would_apply(ctx, &match_glyph))
                    .unwrap_or(false)
            }
            Self::Format2 { input_classes, sets, .. } => {
                let class = input_classes.get(glyph_id);
                sets.get(class.0).map_or(false, |offset| !offset.is_null())
                    && sets.slice(class.0)
                        .and_then(ChainRuleSet::parse)
                        .map(|set| set.would_apply(ctx, &match_class(input_classes)))
                        .unwrap_or(false)
            }
            Self::Format3 { data, backtrack_coverages, input_coverages, lookahead_coverages, .. } => {
                would_apply_chain_context(
                    ctx,
                    backtrack_coverages,
                    input_coverages,
                    lookahead_coverages,
                    &match_coverage(data),
                )
            }
        }
    }

    fn apply(&self, ctx: &mut ApplyContext) -> Option<()> {
        let glyph_id = GlyphId(u16::try_from(ctx.buffer().cur(0).codepoint).unwrap());
        match *self {
            Self::Format1 { coverage, sets } => {
                let index = coverage.get(glyph_id)?;
                let set = ChainRuleSet::parse(sets.slice(index)?)?;
                set.apply(ctx, [&match_glyph, &match_glyph, &match_glyph])
            }
            Self::Format2 {
                coverage,
                backtrack_classes,
                input_classes,
                lookahead_classes,
                sets,
            } => {
                coverage.get(glyph_id)?;
                let class = input_classes.get(glyph_id);
                let offset = sets.get(class.0)?;
                if !offset.is_null() {
                    let set = ChainRuleSet::parse(sets.slice(class.0)?)?;
                    set.apply(ctx, [
                        &match_class(backtrack_classes),
                        &match_class(input_classes),
                        &match_class(lookahead_classes),
                    ])
                } else {
                    None
                }
            }
            Self::Format3 {
                data,
                coverage,
                backtrack_coverages,
                input_coverages,
                lookahead_coverages,
                lookups,
            } => {
                coverage.get(glyph_id)?;
                apply_chain_context(
                    ctx,
                    backtrack_coverages,
                    input_coverages,
                    lookahead_coverages,
                    [
                        &match_coverage(data),
                        &match_coverage(data),
                        &match_coverage(data),
                    ],
                    lookups,
                )
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ChainRuleSet<'a> {
    rules: Offsets16<'a, Offset16>,
}

impl<'a> ChainRuleSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let rules = s.read_offsets16(count, data)?;
        Some(Self { rules })
    }

    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &MatchFunc) -> bool {
        self.rules
            .into_iter()
            .filter_map(|data| ChainRule::parse(data))
            .any(|rules| rules.would_apply(ctx, match_func))
    }

    fn apply(&self, ctx: &mut ApplyContext, match_funcs: [&MatchFunc; 3]) -> Option<()> {
        for data in self.rules {
            if let Some(rule) = ChainRule::parse(data) {
                if rule.apply(ctx, match_funcs).is_some() {
                    return Some(());
                }
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct ChainRule<'a> {
    backtrack: LazyArray16<'a, u16>,
    input: LazyArray16<'a, u16>,
    lookahead: LazyArray16<'a, u16>,
    lookups: LazyArray16<'a, LookupRecord>,
}

impl<'a> ChainRule<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let backtrack_count = s.read::<u16>()?;
        let backtrack = s.read_array16(backtrack_count)?;
        let input_count = s.read::<u16>()?;
        let input = s.read_array16(input_count.checked_sub(1)?)?;
        let lookahead_count = s.read::<u16>()?;
        let lookahead = s.read_array16(lookahead_count)?;
        let lookup_count = s.read::<u16>()?;
        let lookups = s.read_array16(lookup_count)?;
        Some(Self { backtrack, input, lookahead, lookups })
    }

    fn would_apply(&self, ctx: &WouldApplyContext, match_func: &MatchFunc) -> bool {
        would_apply_chain_context(ctx, self.backtrack, self.input, self.lookahead, match_func)
    }

    fn apply(&self, ctx: &mut ApplyContext, match_funcs: [&MatchFunc; 3]) -> Option<()> {
        apply_chain_context(
            ctx,
            self.backtrack,
            self.input,
            self.lookahead,
            match_funcs,
            self.lookups,
        )
    }
}

fn would_apply_context(
    ctx: &WouldApplyContext,
    input: LazyArray16<u16>,
    match_func: &MatchFunc,
) -> bool {
    would_match_input(ctx, input, match_func)
}

fn would_apply_chain_context(
    ctx: &WouldApplyContext,
    backtrack: LazyArray16<u16>,
    input: LazyArray16<u16>,
    lookahead: LazyArray16<u16>,
    match_func: &MatchFunc,
) -> bool {
    (!ctx.zero_context() || (backtrack.len() == 0 && lookahead.len() == 0))
        && would_match_input(ctx, input, match_func)
}

fn apply_context(
    ctx: &mut ApplyContext,
    input: LazyArray16<u16>,
    match_func: &MatchFunc,
    lookups: LazyArray16<LookupRecord>,
) -> Option<()> {
    match_input(ctx, input, match_func).map(|matched| {
        let buffer = ctx.buffer_mut();
        buffer.unsafe_to_break(buffer.idx, buffer.idx + matched.len);
        apply_lookup(ctx, input, matched, lookups);
    })
}

fn apply_chain_context(
    ctx: &mut ApplyContext,
    backtrack: LazyArray16<u16>,
    input: LazyArray16<u16>,
    lookahead: LazyArray16<u16>,
    match_funcs: [&MatchFunc; 3],
    lookups: LazyArray16<LookupRecord>,
) -> Option<()> {
    if let Some(matched) = match_input(ctx, input, match_funcs[1]) {
        if let Some(start_idx) = match_backtrack(ctx, backtrack, match_funcs[0]) {
            if let Some(end_idx) = match_lookahead(ctx, lookahead, match_funcs[2], matched.len) {
                ctx.buffer_mut().unsafe_to_break_from_outbuffer(start_idx, end_idx);
                apply_lookup(ctx, input, matched, lookups);
                return Some(());
            }
        }
    }
    None
}

fn apply_lookup(
    ctx: &mut ApplyContext,
    input: LazyArray16<u16>,
    mut matched: Matched,
    lookups: LazyArray16<LookupRecord>,
) {
    let this_lookup_idx = ctx.lookup_index();
    let mut buffer = ctx.buffer_mut();
    let mut count = 1 + usize::from(input.len());

    // All positions are distance from beginning of *output* buffer.
    // Adjust.
    let mut end = {
        let backtrack_len = buffer.backtrack_len();
        let delta = backtrack_len as isize - buffer.idx as isize;

        // Convert positions to new indexing.
        for j in 0..count {
            matched.positions[j] = (matched.positions[j] as isize + delta) as _;
        }

        backtrack_len + matched.len
    };

    for record in lookups {
        if !buffer.successful {
            break;
        }

        let idx = usize::from(record.sequence_index);
        let lookup_idx = usize::from(record.lookup_list_index);

        if idx >= count {
            continue;
        }

        // Don't recurse to ourself at same position.
        // Note that this test is too naive, it doesn't catch longer loops.
        if idx == 0 && lookup_idx == this_lookup_idx {
            continue;
        }

        if !buffer.move_to(matched.positions[idx]) {
            break;
        }

        if buffer.max_ops <= 0 {
            break;
        }

        let orig_len = buffer.backtrack_len() + buffer.lookahead_len();
        if !ctx.recurse(lookup_idx) {
            buffer = ctx.buffer_mut();
            continue;
        }

        buffer = ctx.buffer_mut();
        let new_len = buffer.backtrack_len() + buffer.lookahead_len();
        let mut delta = new_len as isize - orig_len as isize;
        if delta == 0 {
            continue;
        }

        // Recursed lookup changed buffer len.  Adjust.
        //
        // TODO:
        //
        // Right now, if buffer length increased by n, we assume n new glyphs
        // were added right after the current position, and if buffer length
        // was decreased by n, we assume n match positions after the current
        // one where removed.  The former (buffer length increased) case is
        // fine, but the decrease case can be improved in at least two ways,
        // both of which are significant:
        //
        //   - If recursed-to lookup is MultipleSubst and buffer length
        //     decreased, then it's current match position that was deleted,
        //     NOT the one after it.
        //
        //   - If buffer length was decreased by n, it does not necessarily
        //     mean that n match positions where removed, as there might
        //     have been marks and default-ignorables in the sequence.  We
        //     should instead drop match positions between current-position
        //     and current-position + n instead.
        //
        // It should be possible to construct tests for both of these cases.

        end = (end as isize + delta) as _;
        if end <= matched.positions[idx] {
            // End might end up being smaller than match_positions[idx] if the recursed
            // lookup ended up removing many items, more than we have had matched.
            // Just never rewind end back and get out of here.
            // https://bugs.chromium.org/p/chromium/issues/detail?id=659496
            end = matched.positions[idx];

            // There can't be any further changes.
            break;
        }

        // next now is the position after the recursed lookup.
        let mut next = idx + 1;

        if delta > 0 {
            if delta as usize + count > MAX_CONTEXT_LENGTH {
                break;
            }
        } else {
            // NOTE: delta is negative.
            delta = delta.max(next as isize - count as isize);
            next = (next as isize - delta) as _;
        }

        // Shift!
        matched.positions.copy_within(next .. count, (next as isize + delta) as _);
        next = (next as isize + delta) as _;
        count = (count as isize + delta) as _;

        // Fill in new entries.
        for j in idx+1..next {
            matched.positions[j] = matched.positions[j - 1] + 1;
        }

        // And fixup the rest.
        while next < count {
            matched.positions[next] = (matched.positions[next] as isize + delta) as _;
            next += 1;
        }
    }

    buffer.move_to(end);
}

make_ffi_funcs!(ContextLookup, rb_context_lookup_apply, rb_context_lookup_would_apply);
make_ffi_funcs!(ChainContextLookup, rb_chain_context_lookup_apply, rb_chain_context_lookup_would_apply);