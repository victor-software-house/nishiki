//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

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
use super::grammar::Pat;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::PackageNameAt0 => self.package_name_at0(p),
            Pat::PackageNameAt0End | Pat::ImportAt0End => self.never_anchor_end(p),
            Pat::PackageNameAt0At0
            | Pat::Nested7At0
            | Pat::Nested27At1
            | Pat::Nested28At2
            | Pat::Nested10At1 => self.digit_word(p),
            Pat::PackageNameAt0At1
            | Pat::Nested0At1
            | Pat::Nested3At3
            | Pat::Nested7At1
            | Pat::Nested12At1
            | Pat::Nested13At1
            | Pat::Nested14At5
            | Pat::Nested16At3
            | Pat::Nested17At7
            | Pat::Nested19At1
            | Pat::SingleTypeAt1At9
            | Pat::MultiTypesAt4
            | Pat::Nested20At1
            | Pat::Nested21At2
            | Pat::SwitchTypesAt1At1
            | Pat::Nested24At1
            | Pat::Nested25At6
            | Pat::Nested26At1
            | Pat::Nested27At2
            | Pat::Nested28At3
            | Pat::Nested29At1
            | Pat::Nested30At1
            | Pat::Nested31At3
            | Pat::Nested32At3
            | Pat::SwitchVariablesAt1At0At3
            | Pat::OtherVariables
            | Pat::Nested33At3
            | Pat::Nested18At3
            | Pat::BuiltInFunctionsAt1At5
            | Pat::Nested10At2
            | Pat::Nested9At6
            | Pat::Nested8At1
            | Pat::Nested6At1
            | Pat::MapTypesAt9
            | Pat::Nested2At2 => self.word(p),
            Pat::ImportAt0 => self.import_at0(p),
            Pat::DelimitersAt0 => self.comma(p),
            Pat::DelimitersAt1 => self.delimiters_at1(p),
            Pat::DelimitersAt2 => self.delimiters_at2(p),
            Pat::ImportsAt0 => self.imports_at0(p),
            Pat::ImportsAt1
            | Pat::Nested1At0
            | Pat::FunctionDeclarationAt0
            | Pat::BracketsAt1
            | Pat::Nested9At1
            | Pat::GenericParamTypesAt7
            | Pat::ParameterVariableTypesAt3
            | Pat::FunctionParamTypesAt7
            | Pat::Nested17At1
            | Pat::Nested18At0
            | Pat::SingleTypeAt1At7
            | Pat::Nested25At2
            | Pat::InterfaceVariablesTypesFieldAt3
            | Pat::MapTypesAt7 => self.open_paren(p),
            Pat::ImportsAt1End
            | Pat::Nested1At0End
            | Pat::FunctionDeclarationAt0End
            | Pat::BracketsAt1End
            | Pat::Nested9At1End
            | Pat::GenericParamTypesAt7End
            | Pat::BuiltInFunctionsAt1End
            | Pat::ParameterVariableTypesAt3End
            | Pat::FunctionParamTypesAt7End
            | Pat::BuiltInFunctionsAt2End
            | Pat::Nested17At2
            | Pat::Nested18At0End
            | Pat::SingleTypeAt1At8
            | Pat::MultiTypesEnd
            | Pat::Nested25At3
            | Pat::ConstAssignmentAt1End
            | Pat::VarAssignmentAt1End
            | Pat::InterfaceVariablesTypesFieldAt3End
            | Pat::StructVariableTypesFieldsMultiAt2End
            | Pat::MapTypesAt8 => self.close_paren(p),
            Pat::CommentsAt0 => self.comment_block_begin(p),
            Pat::CommentsAt0End => self.comment_block_end(p),
            Pat::CommentsAt1 => self.comment_line_begin(p),
            Pat::CommentsAt1End | Pat::Nested4At0End => self.line_end(p),
            Pat::SyntaxErrorsAt0 => self.syntax_errors_at0(p),
            Pat::SyntaxErrorsAt1 => self.syntax_errors_at1(p),
            Pat::LanguageConstants => self.language_constants(p),
            Pat::MapTypes => self.map_types(p),
            Pat::KeywordsAt0 => self.keywords_at0(p),
            Pat::KeywordsAt1 => self.chan_word(p),
            Pat::KeywordsAt2 => self.const_word(p),
            Pat::KeywordsAt3 => self.var_word(p),
            Pat::KeywordsAt4 => self.func_word(p),
            Pat::KeywordsAt5 => self.interface_word(p),
            Pat::KeywordsAt6 => self.map_word(p),
            Pat::KeywordsAt7 => self.struct_word(p),
            Pat::KeywordsAt8 => self.import_word(p),
            Pat::KeywordsAt9 | Pat::Nested22At2 => self.type_word(p),
            Pat::OperatorsAt0 => self.operators_at0(p),
            Pat::OperatorsAt1 => self.arrow(p),
            Pat::OperatorsAt2 => self.decrement(p),
            Pat::OperatorsAt3 => self.increment(p),
            Pat::OperatorsAt4 => self.operators_at4(p),
            Pat::OperatorsAt5 => self.operators_at5(p),
            Pat::OperatorsAt6 => self.operators_at6(p),
            Pat::OperatorsAt7 => self.operators_at7(p),
            Pat::OperatorsAt8 => self.operators_at8(p),
            Pat::OperatorsAt9 => self.ellipsis(p),
            Pat::RunesAt0 | Pat::RunesAt0End => self.rune_quote(p),
            Pat::RunesAt0At0 => self.runes_at0_at0(p),
            Pat::RunesAt0At1 => self.runes_at0_at1(p),
            Pat::StorageTypesAt0 => self.bool_word(p),
            Pat::StorageTypesAt1 => self.byte_word(p),
            Pat::StorageTypesAt2 => self.error_word(p),
            Pat::StorageTypesAt3 => self.storage_types_at3(p),
            Pat::StorageTypesAt4 => self.rune_word(p),
            Pat::StorageTypesAt5 => self.string_word(p),
            Pat::StorageTypesAt6 => self.uintptr_word(p),
            Pat::StorageTypesAt7 => self.any_word(p),
            Pat::StorageTypesAt8 => self.comparable_word(p),
            Pat::RawStringLiterals | Pat::RawStringLiteralsEnd => self.backquote(p),
            Pat::StringPlaceholderAt0 => self.string_placeholder_at0(p),
            Pat::StringLiteralsAt0 | Pat::StringLiteralsAt0End => self.double_quote(p),
            Pat::StringEscapedCharAt0 => self.string_escaped_char_at0(p),
            Pat::StringEscapedCharAt1 => self.string_escaped_char_at1(p),
            Pat::Nested4At0 => self.nested4_at0(p),
            Pat::Nested5At0 => self.nested5_at0(p),
            Pat::Nested4At0At0 => self.nested4_at0_at0(p),
            Pat::Nested4At0At1 => self.nested4_at0_at1(p),
            Pat::Nested4At0At2 => self.nested4_at0_at2(p),
            Pat::NumericLiterals => self.numeric_literals(p),
            Pat::Terminators => self.semicolon(p),
            Pat::Nested3At1
            | Pat::BracketsAt2
            | Pat::Nested14At1
            | Pat::Nested16At1
            | Pat::Nested17At3
            | Pat::SingleTypeAt1At3
            | Pat::Nested25At4
            | Pat::Nested9At2
            | Pat::MapTypesAt3 => self.open_square(p),
            Pat::Nested3At2
            | Pat::BracketsAt2End
            | Pat::ParameterVariableTypesAt2End
            | Pat::GenericParamTypesAt6End
            | Pat::FunctionParamTypesAt6End
            | Pat::Nested14At2
            | Pat::Nested16At2
            | Pat::Nested17At4
            | Pat::SingleTypeAt1At4
            | Pat::Nested25At5
            | Pat::InterfaceVariablesTypesFieldAt2End
            | Pat::Nested9At3
            | Pat::FunctionDeclarationAt1End
            | Pat::MapTypesAt4 => self.close_square(p),
            Pat::MapTypesEnd => self.map_types_end(p),
            Pat::ParameterVariableTypesAt0
            | Pat::BracketsAt0
            | Pat::Nested14At3
            | Pat::Nested17At5
            | Pat::SingleTypeAt1At5
            | Pat::Nested9At4
            | Pat::MapTypesAt5 => self.open_curly(p),
            Pat::ParameterVariableTypesAt1
            | Pat::BracketsAt0End
            | Pat::StructVariablesTypesEnd
            | Pat::InterfaceVariablesTypesEnd
            | Pat::Nested14At4
            | Pat::Nested17At6
            | Pat::SingleTypeAt1At6
            | Pat::SwitchTypesEnd
            | Pat::SwitchVariablesAt1End
            | Pat::Nested9At5
            | Pat::StructVariableTypesFieldsMultiAt0End
            | Pat::StructVariableTypesFieldsMultiAt1End
            | Pat::MapTypesAt6 => self.close_curly(p),
            Pat::FunctionDeclaration => self.function_declaration(p),
            Pat::FunctionDeclarationEnd => self.function_declaration_end(p),
            Pat::StructVariablesTypes => self.struct_variables_types(p),
            Pat::InterfaceVariablesTypes => self.interface_variables_types(p),
            Pat::FunctionsInline => self.functions_inline(p),
            Pat::Functions => self.functions(p),
            Pat::FunctionsEnd => self.functions_end(p),
            Pat::ParameterVariableTypesAt2
            | Pat::GenericParamTypesAt6
            | Pat::FunctionParamTypesAt6
            | Pat::InterfaceVariablesTypesFieldAt2
            | Pat::FunctionDeclarationAt1 => self.parameter_variable_types_at2(p),
            Pat::GenericParamTypesAt3 | Pat::FunctionParamTypesAt3 => {
                self.generic_param_types_at3(p)
            }
            Pat::GenericParamTypesAt4 | Pat::FunctionParamTypesAt4 => {
                self.generic_param_types_at4(p)
            }
            Pat::GenericParamTypesAt5 => self.generic_param_types_at5(p),
            Pat::FunctionParamTypesAt5 => self.function_param_types_at5(p),
            Pat::GenericParamTypesAt8 => self.generic_param_types_at8(p),
            Pat::BuiltInFunctionsAt0 => self.built_in_functions_at0(p),
            Pat::BuiltInFunctionsAt1 => self.built_in_functions_at1(p),
            Pat::SupportFunctions => self.support_functions(p),
            Pat::FunctionParamTypesAt8 | Pat::InterfaceVariablesTypesFieldAt4 => {
                self.function_param_types_at8(p)
            }
            Pat::BuiltInFunctionsAt2 => self.built_in_functions_at2(p),
            Pat::AfterControlVariables => self.after_control_variables(p),
            Pat::OtherStructInterfaceExpressionsAt1 => {
                self.other_struct_interface_expressions_at1(p)
            }
            Pat::TypeAssertionInline => self.type_assertion_inline(p),
            Pat::SingleTypeAt0 => self.single_type_at0(p),
            Pat::SingleTypeAt1 => self.single_type_at1(p),
            Pat::SingleTypeAt1End => self.single_type_at1_end(p),
            Pat::MultiTypes => self.multi_types(p),
            Pat::StructInterfaceDeclaration => self.struct_interface_declaration(p),
            Pat::DoubleParenthesesTypes => self.double_parentheses_types(p),
            Pat::SwitchTypes => self.switch_types(p),
            Pat::SwitchTypesAt0 => self.switch_types_at0(p),
            Pat::SwitchTypesAt1 | Pat::SwitchVariablesAt1At0 => self.case_word(p),
            Pat::SwitchTypesAt1End | Pat::SwitchVariablesAt1At0End => self.colon_end(p),
            Pat::GenericTypes => self.generic_types(p),
            Pat::ConstAssignmentAt0 => self.const_assignment_at0(p),
            Pat::ConstAssignmentAt1 => self.const_assignment_at1(p),
            Pat::ConstAssignmentAt1At0 | Pat::VarAssignmentAt1At0 => {
                self.const_assignment_at1_at0(p)
            }
            Pat::VarAssignmentAt0 => self.var_assignment_at0(p),
            Pat::VarAssignmentAt1 => self.var_assignment_at1(p),
            Pat::VariableAssignmentAt0 => self.variable_assignment_at0(p),
            Pat::VariableAssignmentAt1 => self.variable_assignment_at1(p),
            Pat::LabelLoopVariables => self.label_loop_variables(p),
            Pat::SliceIndexVariables => self.slice_index_variables(p),
            Pat::PropertyVariables => self.property_variables(p),
            Pat::SwitchVariablesAt0 => self.switch_variables_at0(p),
            Pat::SwitchVariablesAt1 => self.switch_variables_at1(p),
            Pat::Nested33At0 => self.invalid_type(p),
            Pat::HoverAt0 => self.hover_at0(p),
            Pat::HoverAt1 => self.hover_at1(p),
            Pat::StructVariableTypesFieldsMultiAt0 => {
                self.struct_variable_types_fields_multi_at0(p)
            }
            Pat::StructVariableTypesFieldsMultiAt1 => {
                self.struct_variable_types_fields_multi_at1(p)
            }
            Pat::StructVariableTypesFieldsMultiAt2 => {
                self.struct_variable_types_fields_multi_at2(p)
            }
            Pat::StructVariablesTypesFieldsAt1 => self.struct_variables_types_fields_at1(p),
            Pat::StructVariablesTypesFieldsAt2 => self.struct_variables_types_fields_at2(p),
            Pat::Nested34At0 => self.nested34_at0(p),
            Pat::StructVariablesTypesFieldsAt3 => self.struct_variables_types_fields_at3(p),
            Pat::StructVariablesTypesFieldsAt4 => self.struct_variables_types_fields_at4(p),
            Pat::StructVariablesTypesFieldsAt5 => self.struct_variables_types_fields_at5(p),
            Pat::FunctionDeclarationAt2 => self.function_declaration_at2(p),
            Pat::Nested1At0At0 => self.nested1_at0_at0(p),
        }
    }
}
