//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::cap`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter, or how the matcher gives them back.
//!
//! A regular expression sees the line feed at the end of a line.
//! So `\s` and a negated class such as `[^"]` match it there, and a match can
//! end one past the text.
//! `(?i)` compares words through [`crate::case_fold`], so `ſ` and the Kelvin
//! sign match `s` and `k`.

mod blocks;
mod c4;
mod charts;
mod class;
mod flow;
mod sequence;
mod state;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::case_fold;
use crate::classes::{is_space, is_word};
use crate::rules::Chars;

/// `[-\w]`.
fn is_dash_word(c: char) -> bool {
    c == '-' || is_word(c)
}

/// `[-.\w]`.
fn is_name(c: char) -> bool {
    c == '-' || c == '.' || is_word(c)
}

/// `[!-/:-?\\^\w\s]`: the text of a title or a section.
fn is_text(c: char) -> bool {
    matches!(c, '!'..='/' | ':'..='?' | '\\' | '^') || is_word(c) || is_space(c)
}

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    ///
    /// The arms come in groups, in this order:
    /// - the Markdown blocks, the diagrams' first lines and their ends;
    /// - the C4 diagrams;
    /// - the class diagram;
    /// - the flowchart;
    /// - the sequence diagram;
    /// - the state diagram;
    /// - the other charts.
    #[expect(clippy::too_many_lines, reason = "one arm per pattern of the grammar")]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::MermaidCodeBlock => self.code_block(p, blocks::Fence::Plain),
            Pat::MermaidCodeBlockWithAttributes => self.code_block(p, blocks::Fence::Attributes),
            Pat::MermaidCodeBlockEnd => self.block_end(p, b"`~"),
            Pat::ArchitectureEnd => self.block_end(p, b":`~"),
            Pat::MermaidAdoCodeBlock => self.ado_block(p),
            Pat::MermaidAdoCodeBlockEnd => self.ado_block_end(p),
            Pat::Architecture => self.diagram(p, b"architecture-beta"),
            Pat::C4Component => self.diagram(p, b"C4Component"),
            Pat::C4Container => self.diagram(p, b"C4Container"),
            Pat::C4Deployment => self.diagram(p, b"C4Deployment"),
            Pat::C4Dynamic => self.diagram(p, b"C4Dynamic"),
            Pat::ClassDiagram => self.diagram(p, b"classDiagram"),
            Pat::ErDiagram => self.diagram(p, b"erDiagram"),
            Pat::Gantt => self.diagram(p, b"gantt"),
            Pat::GitGraph => self.diagram(p, b"gitGraph"),
            Pat::Mindmap => self.diagram(p, b"mindmap"),
            Pat::Pie => self.diagram(p, b"pie"),
            Pat::QuadrantChart => self.diagram(p, b"quadrantChart"),
            Pat::RequirementDiagram => self.diagram(p, b"requirementDiagram"),
            Pat::SequenceDiagram => self.diagram(p, b"sequenceDiagram"),
            Pat::Journey => self.diagram(p, b"journey"),
            Pat::StateDiagram => self.state_diagram(p),
            Pat::XyChart => self.xy_chart(p),
            Pat::Flowchart => self.flowchart(p),
            Pat::ArchitectureAt0 => self.comment(p, false),
            Pat::SequenceDiagramAt0 => self.comment(p, true),

            Pat::C4ComponentAt1 => self.title(p),
            Pat::C4ComponentAt2 => self.c4_call(p, c4::PERSON, c4::PERSON_ARGS),
            Pat::C4ComponentAt3 => self.c4_word(p, c4::PERSON),
            Pat::C4ComponentAt4 => self.c4_call(p, c4::SYSTEM, c4::PERSON_ARGS),
            Pat::C4ComponentAt5 => self.c4_word(p, c4::SYSTEM),
            Pat::C4ComponentAt6 => self.c4_call(p, c4::REL, c4::REL_ARGS),
            Pat::C4ComponentAt7 => self.c4_word(p, c4::REL),
            Pat::C4ComponentAt8 => self.c4_call(p, c4::BOUNDARY, c4::BOUNDARY_ARGS),
            Pat::C4ComponentAt9 => self.c4_word(p, c4::BOUNDARY),
            Pat::C4ContainerAt10 => self.c4_call(p, c4::CONTAINER_BOUNDARY, c4::BOUNDARY_ARGS),
            Pat::C4ContainerAt11 => self.c4_word(p, c4::CONTAINER_BOUNDARY),
            Pat::C4ComponentAt10 => self.c4_word(p, c4::LAYOUT),
            Pat::C4ComponentAt11 => self.c4_word(p, &[b"UpdateElementStyle"]),
            Pat::C4ComponentAt12 => self.c4_word(p, &[b"UpdateRelStyle"]),
            Pat::C4ComponentAt13 => self.c4_word(p, c4::TAGS),
            Pat::C4ComponentAt14 => self.c4_call(p, c4::COMPONENT, c4::COMPONENT_ARGS),
            Pat::C4ComponentAt16 => self.c4_word(p, c4::COMPONENT),
            Pat::C4ComponentAt17 => self.c4_word(p, c4::NODE),
            Pat::C4ContainerAt6 => self.c4_call(p, c4::CONTAINER, c4::CONTAINER_ARGS),
            Pat::C4ContainerAt7 => self.c4_word(p, c4::CONTAINER),
            Pat::C4DynamicAt8 => self.c4_call(p, &[b"RelIndex"], c4::REL_INDEX_ARGS),
            Pat::C4DynamicAt9 => self.c4_word(p, &[b"RelIndex"]),

            Pat::ClassDiagramAt1 => self.class_note(p),
            Pat::ClassDiagramAt2 => self.namespace(p),
            Pat::ClassDiagramAt2End => self.namespace_end(p),
            Pat::ClassDiagramAt2At1 => self.class_head(p, class::Head::InNamespace),
            Pat::ClassDiagramAt7 => self.class_head(p, class::Head::Body),
            Pat::ClassDiagramAt8 => self.class_head(p, class::Head::Bare),
            Pat::ClassDiagramAt2At1End => self.group_one(p, b"}"),
            Pat::ClassDiagramAt2At1At1 => self.method_head(p),
            Pat::ClassDiagramAt2At1At1End => self.method_end(p),
            Pat::ClassDiagramAt2At1At1At0 => Some(self.parameter(p)),
            Pat::ClassDiagramAt2At1At2 => self.map_field(p),
            Pat::ClassDiagramAt2At1At3 => self.typed_field(p),
            Pat::ClassDiagramAt2At1At4 => self.field(p),
            Pat::ClassDiagramAt2At1At5 => self.stereotype(p, class::After::Name),
            Pat::ClassDiagramAt6 => self.stereotype(p, class::After::Class),
            Pat::ClassDiagramAt3 => self.relation(p),
            Pat::ClassDiagramAt4 => self.member_method(p),
            Pat::ClassDiagramAt5 => self.member_field(p),

            Pat::FlowchartAt1 => self.subgraph_titled(p),
            Pat::FlowchartAt2 => self.subgraph(p),
            Pat::FlowchartAt3 => self.direction(p, flow::FLOW_DIRECTIONS),
            Pat::FlowchartAt4 => self.end_word(p),
            Pat::FlowchartAt5 => self.node(p),
            Pat::FlowchartAt5End => self.node_end(p),
            Pat::FlowchartAt5At0 => self.quote_open(p),
            Pat::FlowchartAt5At0End => self.group_one(p, b"\""),
            Pat::FlowchartAt5At0At0 | Pat::FlowchartAt5At0At0At0 => Some(self.unquoted(p)),
            Pat::FlowchartAt5At0At0End => (self.at(p) == b'"').then_some(p),
            Pat::FlowchartAt5At1 => self.label(p),
            Pat::FlowchartAt6 => self.piped_link(p),
            Pat::FlowchartAt6End => self.group_one(p, b"|"),
            Pat::FlowchartAt7 => self.texted_link(p),
            Pat::FlowchartAt8 => self.link(p),
            Pat::FlowchartAt9 => self.node_text(p),
            Pat::FlowchartAt10 => self.class_statement(p),
            Pat::FlowchartAt11 => self.class_def(p),
            Pat::FlowchartAt12 => self.click(p),
            Pat::FlowchartAt13 => self.attributes_open(p),
            Pat::FlowchartAt13At0 => self.attribute(p, b"shape"),
            Pat::FlowchartAt13At1 => self.attribute(p, b"label"),

            Pat::SequenceDiagramAt1 => self.sequence_title(p),
            Pat::SequenceDiagramAt2 => self.participant(p),
            Pat::SequenceDiagramAt3 => self.activation(p),
            Pat::SequenceDiagramAt4 => self.note(p),
            Pat::SequenceDiagramAt5 => self.keyword_rest(p, &[b"loop"], sequence::Tail::Open),
            Pat::SequenceDiagramAt6 => self.sequence_end(p),
            Pat::SequenceDiagramAt7 => {
                self.keyword_rest(p, sequence::BLOCKS, sequence::Tail::ToEnd)
            }
            Pat::SequenceDiagramAt8 => self.message(p),
            Pat::SequenceDiagramAt9 => self.transparent_box(p),
            Pat::SequenceDiagramAt10 => self.keyword_rest(p, &[b"box"], sequence::Tail::Open),

            Pat::StateDiagramAt1 => self.direction(p, state::STATE_DIRECTIONS),
            Pat::StateDiagramAt2 => self.spaced(p, b"}"),
            Pat::StateDiagramAt3 => self.spaced(p, b"--"),
            Pat::StateDiagramAt4 => self.lone_state(p),
            Pat::StateDiagramAt5 => self.state_description(p),
            Pat::StateDiagramAt6 => self.state_keyword(p),
            Pat::StateDiagramAt6At0 => self.state_alias(p),
            Pat::StateDiagramAt6At1 => self.composite(p),
            Pat::StateDiagramAt6At2 => self.fork(p),
            Pat::StateDiagramAt7 => self.transition_head(p),
            Pat::StateDiagramAt7At0 => self.transition_target(p),
            Pat::StateDiagramAt7At1 => self.terminal_target(p),
            Pat::StateDiagramAt8 => self.initial_transition(p),
            Pat::StateDiagramAt9 => self.state_note(p),
            Pat::StateDiagramAt10 => self.state_note_block(p),
            Pat::StateDiagramAt10End => self.state_note_end(p),

            Pat::ArchitectureAt1 => self.architecture_item(p),
            Pat::ArchitectureAt2 => self.architecture_edge(p),
            Pat::ArchitectureAt3 => self.junction(p),
            Pat::ErDiagramAt1 => self.entity(p),
            Pat::ErDiagramAt2 => self.entity_open(p),
            Pat::ErDiagramAt2At0 => self.attribute_row(p),
            Pat::ErDiagramAt3 => self.er_relation(p),
            Pat::GanttAt1 => self.setting(p, b"dateFormat", is_name),
            Pat::GanttAt2 => self.setting(p, b"axisFormat", charts::is_axis_format),
            Pat::GanttAt3 => self.tick_interval(p),
            Pat::GanttAt5 => self.excludes(p),
            Pat::GanttAt6 => self.today_marker(p),
            Pat::GanttAt7 => self.titled(p, &[b"section"], charts::Gap::Some),
            Pat::GanttAt8 => self.task(p),
            Pat::GanttAt8End => self.at_eol(p).then_some(p),
            Pat::GanttAt8At0 => charts::TASK_TAGS.iter().find_map(|word| self.lit(p, word)),
            Pat::GitGraphAt1 => self.git_command(p),
            Pat::GitGraphAt1At0 => self.git_option(p, b"id", charts::GitValue::Quoted),
            Pat::GitGraphAt1At1 => self.git_option(p, b"type", charts::GitValue::Type),
            Pat::GitGraphAt1At2 => self.git_option(p, b"tag", charts::GitValue::Tag),
            Pat::GitGraphAt2 => self.checkout(p),
            Pat::GitGraphAt3 => {
                self.branch_or_merge(p, b"branch", b"order", charts::GitValue::Order)
            }
            Pat::GitGraphAt4 => self.branch_or_merge(p, b"merge", b"tag", charts::GitValue::Quoted),
            Pat::GitGraphAt5 => self.cherry_pick(p),
            Pat::MindmapAt1 => self.mindmap_class(p),
            Pat::MindmapAt2 => self.mindmap_icon(p),
            Pat::MindmapAt3 => self.mindmap_shape(p),
            Pat::MindmapAt4 => self.mindmap_text(p),
            Pat::PieAt2 => self.slice(p),
            Pat::QuadrantChartAt1 => self.titled(p, &[b"title"], charts::Gap::Any),
            Pat::QuadrantChartAt2 => self.quadrant_axis(p),
            Pat::QuadrantChartAt2At0 => self.axis_arrow(p),
            Pat::QuadrantChartAt3 => self.quadrant(p),
            Pat::QuadrantChartAt4 => self.point(p),
            Pat::RequirementDiagramAt1 => self.requirement(p),
            Pat::RequirementDiagramAt1End => self.requirement_end(p),
            Pat::RequirementDiagramAt1At0 => self.property(p, b"id:", charts::is_property),
            Pat::RequirementDiagramAt1At1 => self.property(p, b"text:", charts::is_property),
            Pat::RequirementDiagramAt1At2 => self.choice(p, b"risk:", charts::RISKS),
            Pat::RequirementDiagramAt1At3 => self.choice(p, b"verifymethod:", charts::METHODS),
            Pat::RequirementDiagramAt2 => self.element(p),
            Pat::RequirementDiagramAt2At0 => self.property(p, b"type:", charts::is_type_property),
            Pat::RequirementDiagramAt2At1 => self.property(p, b"docref:", charts::is_property),
            Pat::RequirementDiagramAt3 => self.requirement_link(p, b"-", b"->"),
            Pat::RequirementDiagramAt4 => self.requirement_link(p, b"<-", b"-"),
            Pat::JourneyAt1 => self.titled(p, &[b"title", b"section"], charts::Gap::Some),
            Pat::JourneyAt2 => self.journey_task(p),
            Pat::JourneyAt2At0 => self.journey_actor(p),
            Pat::XyChartAt2 => self.axis(p, b"x-axis"),
            Pat::XyChartAt3 => self.axis(p, b"y-axis"),
            Pat::XyChartAt2At0 => self.range(p),
            Pat::XyChartAt2At1 => self.axis_title(p),
            Pat::XyChartAt2At2 => self.axis_word(p),
            Pat::XyChartAt2At3 => self.spaced_open(p, b'['),
            Pat::XyChartAt2At3End => self.spaced_open(p, b']'),
            Pat::XyChartAt2At3At0 => self.spaced_number(p),
            Pat::XyChartAt2At3At1 => self.spaced_string(p),
            Pat::XyChartAt2At3At2 => self.category(p),
            Pat::XyChartAt2At3At3 => self.spaced_open(p, b','),
            Pat::XyChartAt4 => self.series(p),
        }
    }

    /// The character at `p`, with the line feed after the text, and its end.
    fn ch(&self, p: usize) -> Option<(char, usize)> {
        if p < self.lim() {
            self.char_at(p)
        } else if p == self.lim() && self.nl {
            Some(('\n', p + 1))
        } else {
            None
        }
    }

    /// The end of the run of characters `class` holds from `p`.
    fn class_run(&self, p: usize, class: impl Fn(char) -> bool) -> usize {
        let mut q = p;
        while let Some((c, end)) = self.ch(q)
            && class(c)
        {
            q = end;
        }
        self.spend(q - p);
        q
    }

    /// [`Lexer::class_run`] that takes at least one character.
    fn class_run1(&self, p: usize, class: impl Fn(char) -> bool) -> Option<usize> {
        let q = self.class_run(p, class);
        (q > p).then_some(q)
    }

    /// The end of the character at `p` when `class` holds it.
    fn one(&self, p: usize, class: impl Fn(char) -> bool) -> Option<usize> {
        let (c, end) = self.ch(p)?;
        class(c).then_some(end)
    }

    /// `\s*`.
    fn ws(&self, p: usize) -> usize {
        self.space_end(p)
    }

    /// `\s+`.
    fn ws1(&self, p: usize) -> Option<usize> {
        let q = self.space_end(p);
        (q > p).then_some(q)
    }

    /// `\s?`.
    fn ws01(&self, p: usize) -> usize {
        p + self.space_at(p)
    }

    /// The end of `word` at `p` under `(?i)`.
    fn ci(&self, p: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.s, p, word)
    }

    /// The end of the first of `words` at `p` under `(?i)`.
    fn ci_first(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        words.iter().find_map(|word| self.ci(p, word))
    }

    /// `(literal)` at `p`, with the literal its group 1.
    fn group_one(&mut self, p: usize, literal: &[u8]) -> Option<usize> {
        let end = self.lit(p, literal)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `"(?:[^"\\]|\\.)*"`: a string in which a backslash escapes any
    /// character but the line feed.
    /// Each character has one reading, so nothing backtracks.
    fn escaped_string(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'"' {
            return None;
        }
        let mut q = p + 1;
        loop {
            let (c, end) = self.ch(q)?;
            match c {
                '"' => return Some(end),
                '\\' => q = self.ch(end).filter(|&(e, _)| e != '\n')?.1,
                _ => q = end,
            }
        }
    }

    /// `"[^"]*"`, a string that can hold the line feed.
    fn plain_string(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'"' {
            return None;
        }
        let close = self.class_run(p + 1, |c| c != '"');
        (self.at(close) == b'"' && close < self.lim()).then_some(close + 1)
    }

    /// `\s*(C+)` or `\s+(C+)` at `p`, for a class `C` that holds white space:
    /// the span of the group.
    /// The spaces are greedy, so the group starts after them.
    /// Where no character of `C` follows them, the spaces give back the last
    /// one that `C` holds, and the group is that character, as long as the
    /// spaces keep `least` of theirs.
    fn spaced_run(
        &self,
        p: usize,
        least: usize,
        class: impl Fn(char) -> bool + Copy,
    ) -> Option<(usize, usize)> {
        let spaces = self.ws(p);
        if let Some(end) = self.class_run1(spaces, class) {
            return (spaces - p >= least).then_some((spaces, end));
        }
        let mut k = spaces;
        while k > p {
            let start = self.char_start_before(k);
            if start < p + least {
                return None;
            }
            if self.ch(start).is_some_and(|(c, _)| class(c)) {
                return Some((start, self.class_run(start, class)));
            }
            k = start;
        }
        None
    }

    /// `%%.*`, or `(%%|#).*` where `hash` is set: a comment to the end of the
    /// line.
    fn comment(&self, p: usize, hash: bool) -> Option<usize> {
        let starts = self.starts(p, b"%%") || (hash && self.at(p) == b'#');
        starts.then(|| self.lim().max(p))
    }

    /// `(?i)^\s*(title)\s+(\s*[!-/:-?\\^\w\s]*)`: a title.
    fn title(&mut self, p: usize) -> Option<usize> {
        self.titled(p, &[b"title"], charts::Gap::Some)
    }

    /// `^(?i)\s*(direction)\s+(A|B|…)`: a diagram's direction, one of
    /// `directions`.
    fn direction(&mut self, p: usize, directions: &[&[u8]]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.ws(p);
        let word = self.ci(start, b"direction")?;
        let value = self.ws1(word)?;
        let end = self.ci_first(value, directions)?;
        self.cap(1, start, word);
        self.cap(2, value, end);
        Some(end)
    }
}
