//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups of its match with [`Lexer::capture`].
//! The engine names those the grammar names.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
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
use super::grammar::Pattern;
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
    /// Whether `pattern` matches at `p`, and where its match ends.
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
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::MermaidCodeBlock => self.code_block(p, blocks::Fence::Plain),
            Pattern::MermaidCodeBlockWithAttributes => {
                self.code_block(p, blocks::Fence::Attributes)
            }
            Pattern::MermaidCodeBlockEnd => self.block_end(p, b"`~"),
            Pattern::ArchitectureEnd => self.block_end(p, b":`~"),
            Pattern::MermaidAdoCodeBlock => self.ado_block(p),
            Pattern::MermaidAdoCodeBlockEnd => self.ado_block_end(p),
            Pattern::Architecture => self.diagram(p, b"architecture-beta"),
            Pattern::C4Component => self.diagram(p, b"C4Component"),
            Pattern::C4Container => self.diagram(p, b"C4Container"),
            Pattern::C4Deployment => self.diagram(p, b"C4Deployment"),
            Pattern::C4Dynamic => self.diagram(p, b"C4Dynamic"),
            Pattern::ClassDiagram => self.diagram(p, b"classDiagram"),
            Pattern::ErDiagram => self.diagram(p, b"erDiagram"),
            Pattern::Gantt => self.diagram(p, b"gantt"),
            Pattern::GitGraph => self.diagram(p, b"gitGraph"),
            Pattern::Mindmap => self.diagram(p, b"mindmap"),
            Pattern::Pie => self.diagram(p, b"pie"),
            Pattern::QuadrantChart => self.diagram(p, b"quadrantChart"),
            Pattern::RequirementDiagram => self.diagram(p, b"requirementDiagram"),
            Pattern::SequenceDiagram => self.diagram(p, b"sequenceDiagram"),
            Pattern::Journey => self.diagram(p, b"journey"),
            Pattern::StateDiagram => self.state_diagram(p),
            Pattern::XyChart => self.xy_chart(p),
            Pattern::Flowchart => self.flowchart(p),
            Pattern::ArchitectureAt0 => self.comment(p, false),
            Pattern::SequenceDiagramAt0 => self.comment(p, true),

            Pattern::C4ComponentAt1 => self.title(p),
            Pattern::C4ComponentAt2 => self.c4_call(p, c4::PERSON, c4::PERSON_ARGS),
            Pattern::C4ComponentAt3 => self.c4_word(p, c4::PERSON),
            Pattern::C4ComponentAt4 => self.c4_call(p, c4::SYSTEM, c4::PERSON_ARGS),
            Pattern::C4ComponentAt5 => self.c4_word(p, c4::SYSTEM),
            Pattern::C4ComponentAt6 => self.c4_call(p, c4::REL, c4::REL_ARGS),
            Pattern::C4ComponentAt7 => self.c4_word(p, c4::REL),
            Pattern::C4ComponentAt8 => self.c4_call(p, c4::BOUNDARY, c4::BOUNDARY_ARGS),
            Pattern::C4ComponentAt9 => self.c4_word(p, c4::BOUNDARY),
            Pattern::C4ContainerAt10 => self.c4_call(p, c4::CONTAINER_BOUNDARY, c4::BOUNDARY_ARGS),
            Pattern::C4ContainerAt11 => self.c4_word(p, c4::CONTAINER_BOUNDARY),
            Pattern::C4ComponentAt10 => self.c4_word(p, c4::LAYOUT),
            Pattern::C4ComponentAt11 => self.c4_word(p, &[b"UpdateElementStyle"]),
            Pattern::C4ComponentAt12 => self.c4_word(p, &[b"UpdateRelStyle"]),
            Pattern::C4ComponentAt13 => self.c4_word(p, c4::TAGS),
            Pattern::C4ComponentAt14 => self.c4_call(p, c4::COMPONENT, c4::COMPONENT_ARGS),
            Pattern::C4ComponentAt16 => self.c4_word(p, c4::COMPONENT),
            Pattern::C4ComponentAt17 => self.c4_word(p, c4::NODE),
            Pattern::C4ContainerAt6 => self.c4_call(p, c4::CONTAINER, c4::CONTAINER_ARGS),
            Pattern::C4ContainerAt7 => self.c4_word(p, c4::CONTAINER),
            Pattern::C4DynamicAt8 => self.c4_call(p, &[b"RelIndex"], c4::REL_INDEX_ARGS),
            Pattern::C4DynamicAt9 => self.c4_word(p, &[b"RelIndex"]),

            Pattern::ClassDiagramAt1 => self.class_note(p),
            Pattern::ClassDiagramAt2 => self.namespace(p),
            Pattern::ClassDiagramAt2End => self.namespace_end(p),
            Pattern::ClassDiagramAt2At1 => self.class_head(p, class::Head::InNamespace),
            Pattern::ClassDiagramAt7 => self.class_head(p, class::Head::Body),
            Pattern::ClassDiagramAt8 => self.class_head(p, class::Head::Bare),
            Pattern::ClassDiagramAt2At1End => self.group_one(p, b"}"),
            Pattern::ClassDiagramAt2At1At1 => self.method_head(p),
            Pattern::ClassDiagramAt2At1At1End => self.method_end(p),
            Pattern::ClassDiagramAt2At1At1At0 => Some(self.parameter(p)),
            Pattern::ClassDiagramAt2At1At2 => self.map_field(p),
            Pattern::ClassDiagramAt2At1At3 => self.typed_field(p),
            Pattern::ClassDiagramAt2At1At4 => self.field(p),
            Pattern::ClassDiagramAt2At1At5 => self.stereotype(p, class::After::Name),
            Pattern::ClassDiagramAt6 => self.stereotype(p, class::After::Class),
            Pattern::ClassDiagramAt3 => self.relation(p),
            Pattern::ClassDiagramAt4 => self.member_method(p),
            Pattern::ClassDiagramAt5 => self.member_field(p),

            Pattern::FlowchartAt1 => self.subgraph_titled(p),
            Pattern::FlowchartAt2 => self.subgraph(p),
            Pattern::FlowchartAt3 => self.direction(p, flow::FLOW_DIRECTIONS),
            Pattern::FlowchartAt4 => self.end_word(p),
            Pattern::FlowchartAt5 => self.node(p),
            Pattern::FlowchartAt5End => self.node_end(p),
            Pattern::FlowchartAt5At0 => self.quote_open(p),
            Pattern::FlowchartAt5At0End => self.group_one(p, b"\""),
            Pattern::FlowchartAt5At0At0 | Pattern::FlowchartAt5At0At0At0 => Some(self.unquoted(p)),
            Pattern::FlowchartAt5At0At0End => (self.at(p) == b'"').then_some(p),
            Pattern::FlowchartAt5At1 => self.label(p),
            Pattern::FlowchartAt6 => self.piped_link(p),
            Pattern::FlowchartAt6End => self.group_one(p, b"|"),
            Pattern::FlowchartAt7 => self.texted_link(p),
            Pattern::FlowchartAt8 => self.link(p),
            Pattern::FlowchartAt9 => self.node_text(p),
            Pattern::FlowchartAt10 => self.class_statement(p),
            Pattern::FlowchartAt11 => self.class_def(p),
            Pattern::FlowchartAt12 => self.click(p),
            Pattern::FlowchartAt13 => self.attributes_open(p),
            Pattern::FlowchartAt13At0 => self.attribute(p, b"shape"),
            Pattern::FlowchartAt13At1 => self.attribute(p, b"label"),

            Pattern::SequenceDiagramAt1 => self.sequence_title(p),
            Pattern::SequenceDiagramAt2 => self.participant(p),
            Pattern::SequenceDiagramAt3 => self.activation(p),
            Pattern::SequenceDiagramAt4 => self.note(p),
            Pattern::SequenceDiagramAt5 => self.keyword_rest(p, &[b"loop"], sequence::Tail::Open),
            Pattern::SequenceDiagramAt6 => self.sequence_end(p),
            Pattern::SequenceDiagramAt7 => {
                self.keyword_rest(p, sequence::BLOCKS, sequence::Tail::ToEnd)
            }
            Pattern::SequenceDiagramAt8 => self.message(p),
            Pattern::SequenceDiagramAt9 => self.transparent_box(p),
            Pattern::SequenceDiagramAt10 => self.keyword_rest(p, &[b"box"], sequence::Tail::Open),

            Pattern::StateDiagramAt1 => self.direction(p, state::STATE_DIRECTIONS),
            Pattern::StateDiagramAt2 => self.spaced(p, b"}"),
            Pattern::StateDiagramAt3 => self.spaced(p, b"--"),
            Pattern::StateDiagramAt4 => self.lone_state(p),
            Pattern::StateDiagramAt5 => self.state_description(p),
            Pattern::StateDiagramAt6 => self.state_keyword(p),
            Pattern::StateDiagramAt6At0 => self.state_alias(p),
            Pattern::StateDiagramAt6At1 => self.composite(p),
            Pattern::StateDiagramAt6At2 => self.fork(p),
            Pattern::StateDiagramAt7 => self.transition_head(p),
            Pattern::StateDiagramAt7At0 => self.transition_target(p),
            Pattern::StateDiagramAt7At1 => self.terminal_target(p),
            Pattern::StateDiagramAt8 => self.initial_transition(p),
            Pattern::StateDiagramAt9 => self.state_note(p),
            Pattern::StateDiagramAt10 => self.state_note_block(p),
            Pattern::StateDiagramAt10End => self.state_note_end(p),

            Pattern::ArchitectureAt1 => self.architecture_item(p),
            Pattern::ArchitectureAt2 => self.architecture_edge(p),
            Pattern::ArchitectureAt3 => self.junction(p),
            Pattern::ErDiagramAt1 => self.entity(p),
            Pattern::ErDiagramAt2 => self.entity_open(p),
            Pattern::ErDiagramAt2At0 => self.attribute_row(p),
            Pattern::ErDiagramAt3 => self.er_relation(p),
            Pattern::GanttAt1 => self.setting(p, b"dateFormat", is_name),
            Pattern::GanttAt2 => self.setting(p, b"axisFormat", charts::is_axis_format),
            Pattern::GanttAt3 => self.tick_interval(p),
            Pattern::GanttAt5 => self.excludes(p),
            Pattern::GanttAt6 => self.today_marker(p),
            Pattern::GanttAt7 => self.titled(p, &[b"section"], charts::Gap::Some),
            Pattern::GanttAt8 => self.task(p),
            Pattern::GanttAt8End => self.at_eol(p).then_some(p),
            Pattern::GanttAt8At0 => charts::TASK_TAGS.iter().find_map(|word| self.lit(p, word)),
            Pattern::GitGraphAt1 => self.git_command(p),
            Pattern::GitGraphAt1At0 => self.git_option(p, b"id", charts::GitValue::Quoted),
            Pattern::GitGraphAt1At1 => self.git_option(p, b"type", charts::GitValue::Type),
            Pattern::GitGraphAt1At2 => self.git_option(p, b"tag", charts::GitValue::Tag),
            Pattern::GitGraphAt2 => self.checkout(p),
            Pattern::GitGraphAt3 => {
                self.branch_or_merge(p, b"branch", b"order", charts::GitValue::Order)
            }
            Pattern::GitGraphAt4 => {
                self.branch_or_merge(p, b"merge", b"tag", charts::GitValue::Quoted)
            }
            Pattern::GitGraphAt5 => self.cherry_pick(p),
            Pattern::MindmapAt1 => self.mindmap_class(p),
            Pattern::MindmapAt2 => self.mindmap_icon(p),
            Pattern::MindmapAt3 => self.mindmap_shape(p),
            Pattern::MindmapAt4 => self.mindmap_text(p),
            Pattern::PieAt2 => self.slice(p),
            Pattern::QuadrantChartAt1 => self.titled(p, &[b"title"], charts::Gap::Any),
            Pattern::QuadrantChartAt2 => self.quadrant_axis(p),
            Pattern::QuadrantChartAt2At0 => self.axis_arrow(p),
            Pattern::QuadrantChartAt3 => self.quadrant(p),
            Pattern::QuadrantChartAt4 => self.point(p),
            Pattern::RequirementDiagramAt1 => self.requirement(p),
            Pattern::RequirementDiagramAt1End => self.requirement_end(p),
            Pattern::RequirementDiagramAt1At0 => self.property(p, b"id:", charts::is_property),
            Pattern::RequirementDiagramAt1At1 => self.property(p, b"text:", charts::is_property),
            Pattern::RequirementDiagramAt1At2 => self.choice(p, b"risk:", charts::RISKS),
            Pattern::RequirementDiagramAt1At3 => self.choice(p, b"verifymethod:", charts::METHODS),
            Pattern::RequirementDiagramAt2 => self.element(p),
            Pattern::RequirementDiagramAt2At0 => {
                self.property(p, b"type:", charts::is_type_property)
            }
            Pattern::RequirementDiagramAt2At1 => self.property(p, b"docref:", charts::is_property),
            Pattern::RequirementDiagramAt3 => self.requirement_link(p, b"-", b"->"),
            Pattern::RequirementDiagramAt4 => self.requirement_link(p, b"<-", b"-"),
            Pattern::JourneyAt1 => self.titled(p, &[b"title", b"section"], charts::Gap::Some),
            Pattern::JourneyAt2 => self.journey_task(p),
            Pattern::JourneyAt2At0 => self.journey_actor(p),
            Pattern::XyChartAt2 => self.axis(p, b"x-axis"),
            Pattern::XyChartAt3 => self.axis(p, b"y-axis"),
            Pattern::XyChartAt2At0 => self.range(p),
            Pattern::XyChartAt2At1 => self.axis_title(p),
            Pattern::XyChartAt2At2 => self.axis_word(p),
            Pattern::XyChartAt2At3 => self.spaced_open(p, b'['),
            Pattern::XyChartAt2At3End => self.spaced_open(p, b']'),
            Pattern::XyChartAt2At3At0 => self.spaced_number(p),
            Pattern::XyChartAt2At3At1 => self.spaced_string(p),
            Pattern::XyChartAt2At3At2 => self.category(p),
            Pattern::XyChartAt2At3At3 => self.spaced_open(p, b','),
            Pattern::XyChartAt4 => self.series(p),
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
        self.capture(1, p, end);
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
    /// one that `C` holds.
    /// The group is then that character, as long as the spaces keep `least`
    /// of theirs.
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
        self.capture(1, start, word);
        self.capture(2, value, end);
        Some(end)
    }
}
