//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter.

mod chars;
mod fields;
mod functions;
mod hot;
mod numbers;
mod params;
mod simple;
mod statements;
mod strings;
mod types;

use super::engine::Lexer;
use super::grammar::Pattern;

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::PackageNameAt0 => self.package_name_at0(p),
            Pattern::PackageNameAt0End | Pattern::ImportAt0End => self.never_anchor_end(p),
            Pattern::PackageNameAt0At0
            | Pattern::Nested7At0
            | Pattern::Nested27At1
            | Pattern::Nested28At2
            | Pattern::Nested10At1 => self.digit_word(p),
            Pattern::PackageNameAt0At1
            | Pattern::Nested0At1
            | Pattern::Nested3At3
            | Pattern::Nested7At1
            | Pattern::Nested12At1
            | Pattern::Nested13At1
            | Pattern::Nested14At5
            | Pattern::Nested16At3
            | Pattern::Nested17At7
            | Pattern::Nested19At1
            | Pattern::SingleTypeAt1At9
            | Pattern::MultiTypesAt4
            | Pattern::Nested20At1
            | Pattern::Nested21At2
            | Pattern::SwitchTypesAt1At1
            | Pattern::Nested24At1
            | Pattern::Nested25At6
            | Pattern::Nested26At1
            | Pattern::Nested27At2
            | Pattern::Nested28At3
            | Pattern::Nested29At1
            | Pattern::Nested30At1
            | Pattern::Nested31At3
            | Pattern::Nested32At3
            | Pattern::SwitchVariablesAt1At0At3
            | Pattern::OtherVariables
            | Pattern::Nested33At3
            | Pattern::Nested18At3
            | Pattern::BuiltInFunctionsAt1At5
            | Pattern::Nested10At2
            | Pattern::Nested9At6
            | Pattern::Nested8At1
            | Pattern::Nested6At1
            | Pattern::MapTypesAt9
            | Pattern::Nested2At2 => self.word(p),
            Pattern::ImportAt0 => self.import_at0(p),
            Pattern::DelimitersAt0 => self.comma(p),
            Pattern::DelimitersAt1 => self.delimiters_at1(p),
            Pattern::DelimitersAt2 => self.delimiters_at2(p),
            Pattern::ImportsAt0 => self.imports_at0(p),
            Pattern::ImportsAt1
            | Pattern::Nested1At0
            | Pattern::FunctionDeclarationAt0
            | Pattern::BracketsAt1
            | Pattern::Nested9At1
            | Pattern::GenericParamTypesAt7
            | Pattern::ParameterVariableTypesAt3
            | Pattern::FunctionParamTypesAt7
            | Pattern::Nested17At1
            | Pattern::Nested18At0
            | Pattern::SingleTypeAt1At7
            | Pattern::Nested25At2
            | Pattern::InterfaceVariablesTypesFieldAt3
            | Pattern::MapTypesAt7 => self.open_paren(p),
            Pattern::ImportsAt1End
            | Pattern::Nested1At0End
            | Pattern::FunctionDeclarationAt0End
            | Pattern::BracketsAt1End
            | Pattern::Nested9At1End
            | Pattern::GenericParamTypesAt7End
            | Pattern::BuiltInFunctionsAt1End
            | Pattern::ParameterVariableTypesAt3End
            | Pattern::FunctionParamTypesAt7End
            | Pattern::BuiltInFunctionsAt2End
            | Pattern::Nested17At2
            | Pattern::Nested18At0End
            | Pattern::SingleTypeAt1At8
            | Pattern::MultiTypesEnd
            | Pattern::Nested25At3
            | Pattern::ConstAssignmentAt1End
            | Pattern::VarAssignmentAt1End
            | Pattern::InterfaceVariablesTypesFieldAt3End
            | Pattern::StructVariableTypesFieldsMultiAt2End
            | Pattern::MapTypesAt8 => self.close_paren(p),
            Pattern::CommentsAt0 => self.comment_block_begin(p),
            Pattern::CommentsAt0End => self.comment_block_end(p),
            Pattern::CommentsAt1 => self.comment_line_begin(p),
            Pattern::CommentsAt1End | Pattern::Nested4At0End => self.line_end(p),
            Pattern::SyntaxErrorsAt0 => self.syntax_errors_at0(p),
            Pattern::SyntaxErrorsAt1 => self.syntax_errors_at1(p),
            Pattern::LanguageConstants => self.language_constants(p),
            Pattern::MapTypes => self.map_types(p),
            Pattern::KeywordsAt0 => self.keywords_at0(p),
            Pattern::KeywordsAt1 => self.chan_word(p),
            Pattern::KeywordsAt2 => self.const_word(p),
            Pattern::KeywordsAt3 => self.var_word(p),
            Pattern::KeywordsAt4 => self.func_word(p),
            Pattern::KeywordsAt5 => self.interface_word(p),
            Pattern::KeywordsAt6 => self.map_word(p),
            Pattern::KeywordsAt7 => self.struct_word(p),
            Pattern::KeywordsAt8 => self.import_word(p),
            Pattern::KeywordsAt9 | Pattern::Nested22At2 => self.type_word(p),
            Pattern::OperatorsAt0 => self.operators_at0(p),
            Pattern::OperatorsAt1 => self.arrow(p),
            Pattern::OperatorsAt2 => self.decrement(p),
            Pattern::OperatorsAt3 => self.increment(p),
            Pattern::OperatorsAt4 => self.operators_at4(p),
            Pattern::OperatorsAt5 => self.operators_at5(p),
            Pattern::OperatorsAt6 => self.operators_at6(p),
            Pattern::OperatorsAt7 => self.operators_at7(p),
            Pattern::OperatorsAt8 => self.operators_at8(p),
            Pattern::OperatorsAt9 => self.ellipsis(p),
            Pattern::RunesAt0 | Pattern::RunesAt0End => self.rune_quote(p),
            Pattern::RunesAt0At0 => self.runes_at0_at0(p),
            Pattern::RunesAt0At1 => self.runes_at0_at1(p),
            Pattern::StorageTypesAt0 => self.bool_word(p),
            Pattern::StorageTypesAt1 => self.byte_word(p),
            Pattern::StorageTypesAt2 => self.error_word(p),
            Pattern::StorageTypesAt3 => self.storage_types_at3(p),
            Pattern::StorageTypesAt4 => self.rune_word(p),
            Pattern::StorageTypesAt5 => self.string_word(p),
            Pattern::StorageTypesAt6 => self.uintptr_word(p),
            Pattern::StorageTypesAt7 => self.any_word(p),
            Pattern::StorageTypesAt8 => self.comparable_word(p),
            Pattern::RawStringLiterals | Pattern::RawStringLiteralsEnd => self.backquote(p),
            Pattern::StringPlaceholderAt0 => self.string_placeholder_at0(p),
            Pattern::StringLiteralsAt0 | Pattern::StringLiteralsAt0End => self.double_quote(p),
            Pattern::StringEscapedCharAt0 => self.string_escaped_char_at0(p),
            Pattern::StringEscapedCharAt1 => self.string_escaped_char_at1(p),
            Pattern::Nested4At0 => self.nested4_at0(p),
            Pattern::Nested5At0 => self.nested5_at0(p),
            Pattern::Nested4At0At0 => self.nested4_at0_at0(p),
            Pattern::Nested4At0At1 => self.nested4_at0_at1(p),
            Pattern::Nested4At0At2 => self.nested4_at0_at2(p),
            Pattern::NumericLiterals => self.numeric_literals(p),
            Pattern::Terminators => self.semicolon(p),
            Pattern::Nested3At1
            | Pattern::BracketsAt2
            | Pattern::Nested14At1
            | Pattern::Nested16At1
            | Pattern::Nested17At3
            | Pattern::SingleTypeAt1At3
            | Pattern::Nested25At4
            | Pattern::Nested9At2
            | Pattern::MapTypesAt3 => self.open_square(p),
            Pattern::Nested3At2
            | Pattern::BracketsAt2End
            | Pattern::ParameterVariableTypesAt2End
            | Pattern::GenericParamTypesAt6End
            | Pattern::FunctionParamTypesAt6End
            | Pattern::Nested14At2
            | Pattern::Nested16At2
            | Pattern::Nested17At4
            | Pattern::SingleTypeAt1At4
            | Pattern::Nested25At5
            | Pattern::InterfaceVariablesTypesFieldAt2End
            | Pattern::Nested9At3
            | Pattern::FunctionDeclarationAt1End
            | Pattern::MapTypesAt4 => self.close_square(p),
            Pattern::MapTypesEnd => self.map_types_end(p),
            Pattern::ParameterVariableTypesAt0
            | Pattern::BracketsAt0
            | Pattern::Nested14At3
            | Pattern::Nested17At5
            | Pattern::SingleTypeAt1At5
            | Pattern::Nested9At4
            | Pattern::MapTypesAt5 => self.open_curly(p),
            Pattern::ParameterVariableTypesAt1
            | Pattern::BracketsAt0End
            | Pattern::StructVariablesTypesEnd
            | Pattern::InterfaceVariablesTypesEnd
            | Pattern::Nested14At4
            | Pattern::Nested17At6
            | Pattern::SingleTypeAt1At6
            | Pattern::SwitchTypesEnd
            | Pattern::SwitchVariablesAt1End
            | Pattern::Nested9At5
            | Pattern::StructVariableTypesFieldsMultiAt0End
            | Pattern::StructVariableTypesFieldsMultiAt1End
            | Pattern::MapTypesAt6 => self.close_curly(p),
            Pattern::FunctionDeclaration => self.function_declaration(p),
            Pattern::FunctionDeclarationEnd => self.function_declaration_end(p),
            Pattern::StructVariablesTypes => self.struct_variables_types(p),
            Pattern::InterfaceVariablesTypes => self.interface_variables_types(p),
            Pattern::FunctionsInline => self.functions_inline(p),
            Pattern::Functions => self.functions(p),
            Pattern::FunctionsEnd => self.functions_end(p),
            Pattern::ParameterVariableTypesAt2
            | Pattern::GenericParamTypesAt6
            | Pattern::FunctionParamTypesAt6
            | Pattern::InterfaceVariablesTypesFieldAt2
            | Pattern::FunctionDeclarationAt1 => self.parameter_variable_types_at2(p),
            Pattern::GenericParamTypesAt3 | Pattern::FunctionParamTypesAt3 => {
                self.generic_param_types_at3(p)
            }
            Pattern::GenericParamTypesAt4 | Pattern::FunctionParamTypesAt4 => {
                self.generic_param_types_at4(p)
            }
            Pattern::GenericParamTypesAt5 => self.generic_param_types_at5(p),
            Pattern::FunctionParamTypesAt5 => self.function_param_types_at5(p),
            Pattern::GenericParamTypesAt8 => self.generic_param_types_at8(p),
            Pattern::BuiltInFunctionsAt0 => self.built_in_functions_at0(p),
            Pattern::BuiltInFunctionsAt1 => self.built_in_functions_at1(p),
            Pattern::SupportFunctions => self.support_functions(p),
            Pattern::FunctionParamTypesAt8 | Pattern::InterfaceVariablesTypesFieldAt4 => {
                self.function_param_types_at8(p)
            }
            Pattern::BuiltInFunctionsAt2 => self.built_in_functions_at2(p),
            Pattern::AfterControlVariables => self.after_control_variables(p),
            Pattern::OtherStructInterfaceExpressionsAt1 => {
                self.other_struct_interface_expressions_at1(p)
            }
            Pattern::TypeAssertionInline => self.type_assertion_inline(p),
            Pattern::SingleTypeAt0 => self.single_type_at0(p),
            Pattern::SingleTypeAt1 => self.single_type_at1(p),
            Pattern::SingleTypeAt1End => self.single_type_at1_end(p),
            Pattern::MultiTypes => self.multi_types(p),
            Pattern::StructInterfaceDeclaration => self.struct_interface_declaration(p),
            Pattern::DoubleParenthesesTypes => self.double_parentheses_types(p),
            Pattern::SwitchTypes => self.switch_types(p),
            Pattern::SwitchTypesAt0 => self.switch_types_at0(p),
            Pattern::SwitchTypesAt1 | Pattern::SwitchVariablesAt1At0 => self.case_word(p),
            Pattern::SwitchTypesAt1End | Pattern::SwitchVariablesAt1At0End => self.colon_end(p),
            Pattern::GenericTypes => self.generic_types(p),
            Pattern::ConstAssignmentAt0 => self.const_assignment_at0(p),
            Pattern::ConstAssignmentAt1 => self.const_assignment_at1(p),
            Pattern::ConstAssignmentAt1At0 | Pattern::VarAssignmentAt1At0 => {
                self.const_assignment_at1_at0(p)
            }
            Pattern::VarAssignmentAt0 => self.var_assignment_at0(p),
            Pattern::VarAssignmentAt1 => self.var_assignment_at1(p),
            Pattern::VariableAssignmentAt0 => self.variable_assignment_at0(p),
            Pattern::VariableAssignmentAt1 => self.variable_assignment_at1(p),
            Pattern::LabelLoopVariables => self.label_loop_variables(p),
            Pattern::SliceIndexVariables => self.slice_index_variables(p),
            Pattern::PropertyVariables => self.property_variables(p),
            Pattern::SwitchVariablesAt0 => self.switch_variables_at0(p),
            Pattern::SwitchVariablesAt1 => self.switch_variables_at1(p),
            Pattern::Nested33At0 => self.invalid_type(p),
            Pattern::HoverAt0 => self.hover_at0(p),
            Pattern::HoverAt1 => self.hover_at1(p),
            Pattern::StructVariableTypesFieldsMultiAt0 => {
                self.struct_variable_types_fields_multi_at0(p)
            }
            Pattern::StructVariableTypesFieldsMultiAt1 => {
                self.struct_variable_types_fields_multi_at1(p)
            }
            Pattern::StructVariableTypesFieldsMultiAt2 => {
                self.struct_variable_types_fields_multi_at2(p)
            }
            Pattern::StructVariablesTypesFieldsAt1 => self.struct_variables_types_fields_at1(p),
            Pattern::StructVariablesTypesFieldsAt2 => self.struct_variables_types_fields_at2(p),
            Pattern::Nested34At0 => self.nested34_at0(p),
            Pattern::StructVariablesTypesFieldsAt3 => self.struct_variables_types_fields_at3(p),
            Pattern::StructVariablesTypesFieldsAt4 => self.struct_variables_types_fields_at4(p),
            Pattern::StructVariablesTypesFieldsAt5 => self.struct_variables_types_fields_at5(p),
            Pattern::FunctionDeclarationAt2 => self.function_declaration_at2(p),
            Pattern::Nested1At0At0 => self.nested1_at0_at0(p),
        }
    }
}
