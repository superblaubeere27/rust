//! Errors emitted by typeck.
use rustc_errors::{error_code, Applicability, DiagnosticBuilder, DiagnosticId, ErrorGuaranteed};
use rustc_macros::{LintDiagnostic, SessionDiagnostic, SessionSubdiagnostic};
use rustc_middle::ty::Ty;
use rustc_session::{parse::ParseSess, SessionDiagnostic};
use rustc_span::{symbol::Ident, Span, Symbol};

#[derive(SessionDiagnostic)]
#[diag(typeck::field_multiply_specified_in_initializer, code = "E0062")]
pub struct FieldMultiplySpecifiedInInitializer {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(typeck::previous_use_label)]
    pub prev_span: Span,
    pub ident: Ident,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::unrecognized_atomic_operation, code = "E0092")]
pub struct UnrecognizedAtomicOperation<'a> {
    #[primary_span]
    #[label]
    pub span: Span,
    pub op: &'a str,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::wrong_number_of_generic_arguments_to_intrinsic, code = "E0094")]
pub struct WrongNumberOfGenericArgumentsToIntrinsic<'a> {
    #[primary_span]
    #[label]
    pub span: Span,
    pub found: usize,
    pub expected: usize,
    pub descr: &'a str,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::unrecognized_intrinsic_function, code = "E0093")]
pub struct UnrecognizedIntrinsicFunction {
    #[primary_span]
    #[label]
    pub span: Span,
    pub name: Symbol,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::lifetimes_or_bounds_mismatch_on_trait, code = "E0195")]
pub struct LifetimesOrBoundsMismatchOnTrait {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(typeck::generics_label)]
    pub generics_span: Option<Span>,
    pub item_kind: &'static str,
    pub ident: Ident,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::drop_impl_on_wrong_item, code = "E0120")]
pub struct DropImplOnWrongItem {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::field_already_declared, code = "E0124")]
pub struct FieldAlreadyDeclared {
    pub field_name: Ident,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(typeck::previous_decl_label)]
    pub prev_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::copy_impl_on_type_with_dtor, code = "E0184")]
pub struct CopyImplOnTypeWithDtor {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::multiple_relaxed_default_bounds, code = "E0203")]
pub struct MultipleRelaxedDefaultBounds {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::copy_impl_on_non_adt, code = "E0206")]
pub struct CopyImplOnNonAdt {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::trait_object_declared_with_no_traits, code = "E0224")]
pub struct TraitObjectDeclaredWithNoTraits {
    #[primary_span]
    pub span: Span,
    #[label(typeck::alias_span)]
    pub trait_alias_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::ambiguous_lifetime_bound, code = "E0227")]
pub struct AmbiguousLifetimeBound {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::assoc_type_binding_not_allowed, code = "E0229")]
pub struct AssocTypeBindingNotAllowed {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::functional_record_update_on_non_struct, code = "E0436")]
pub struct FunctionalRecordUpdateOnNonStruct {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::typeof_reserved_keyword_used, code = "E0516")]
pub struct TypeofReservedKeywordUsed<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
    #[suggestion_verbose(code = "{ty}")]
    pub opt_sugg: Option<(Span, Applicability)>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::return_stmt_outside_of_fn_body, code = "E0572")]
pub struct ReturnStmtOutsideOfFnBody {
    #[primary_span]
    pub span: Span,
    #[label(typeck::encl_body_label)]
    pub encl_body_span: Option<Span>,
    #[label(typeck::encl_fn_label)]
    pub encl_fn_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::yield_expr_outside_of_generator, code = "E0627")]
pub struct YieldExprOutsideOfGenerator {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::struct_expr_non_exhaustive, code = "E0639")]
pub struct StructExprNonExhaustive {
    #[primary_span]
    pub span: Span,
    pub what: &'static str,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::method_call_on_unknown_type, code = "E0699")]
pub struct MethodCallOnUnknownType {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::value_of_associated_struct_already_specified, code = "E0719")]
pub struct ValueOfAssociatedStructAlreadySpecified {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(typeck::previous_bound_label)]
    pub prev_span: Span,
    pub item_name: Ident,
    pub def_path: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::address_of_temporary_taken, code = "E0745")]
pub struct AddressOfTemporaryTaken {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionSubdiagnostic)]
pub enum AddReturnTypeSuggestion<'tcx> {
    #[suggestion(
        typeck::add_return_type_add,
        code = "-> {found} ",
        applicability = "machine-applicable"
    )]
    Add {
        #[primary_span]
        span: Span,
        found: Ty<'tcx>,
    },
    #[suggestion(
        typeck::add_return_type_missing_here,
        code = "-> _ ",
        applicability = "has-placeholders"
    )]
    MissingHere {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub enum ExpectedReturnTypeLabel<'tcx> {
    #[label(typeck::expected_default_return_type)]
    Unit {
        #[primary_span]
        span: Span,
    },
    #[label(typeck::expected_return_type)]
    Other {
        #[primary_span]
        span: Span,
        expected: Ty<'tcx>,
    },
}

#[derive(SessionDiagnostic)]
#[diag(typeck::unconstrained_opaque_type)]
#[note]
pub struct UnconstrainedOpaqueType {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}

pub struct MissingTypeParams {
    pub span: Span,
    pub def_span: Span,
    pub missing_type_params: Vec<Symbol>,
    pub empty_generic_args: bool,
}

// Manual implementation of `SessionDiagnostic` to be able to call `span_to_snippet`.
impl<'a> SessionDiagnostic<'a> for MissingTypeParams {
    fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
        let mut err = sess.span_diagnostic.struct_span_err_with_code(
            self.span,
            rustc_errors::fluent::typeck::missing_type_params,
            error_code!(E0393),
        );
        err.set_arg("parameterCount", self.missing_type_params.len());
        err.set_arg(
            "parameters",
            self.missing_type_params
                .iter()
                .map(|n| format!("`{}`", n))
                .collect::<Vec<_>>()
                .join(", "),
        );

        err.span_label(self.def_span, rustc_errors::fluent::typeck::label);

        let mut suggested = false;
        if let (Ok(snippet), true) = (
            sess.source_map().span_to_snippet(self.span),
            // Don't suggest setting the type params if there are some already: the order is
            // tricky to get right and the user will already know what the syntax is.
            self.empty_generic_args,
        ) {
            if snippet.ends_with('>') {
                // The user wrote `Trait<'a, T>` or similar. To provide an accurate suggestion
                // we would have to preserve the right order. For now, as clearly the user is
                // aware of the syntax, we do nothing.
            } else {
                // The user wrote `Iterator`, so we don't have a type we can suggest, but at
                // least we can clue them to the correct syntax `Iterator<Type>`.
                err.span_suggestion(
                    self.span,
                    rustc_errors::fluent::typeck::suggestion,
                    format!(
                        "{}<{}>",
                        snippet,
                        self.missing_type_params
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    Applicability::HasPlaceholders,
                );
                suggested = true;
            }
        }
        if !suggested {
            err.span_label(self.span, rustc_errors::fluent::typeck::no_suggestion_label);
        }

        err.note(rustc_errors::fluent::typeck::note);
        err
    }
}

#[derive(SessionDiagnostic)]
#[diag(typeck::manual_implementation, code = "E0183")]
#[help]
pub struct ManualImplementation {
    #[primary_span]
    #[label]
    pub span: Span,
    pub trait_name: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::substs_on_overridden_impl)]
pub struct SubstsOnOverriddenImpl {
    #[primary_span]
    pub span: Span,
}

#[derive(LintDiagnostic)]
#[diag(typeck::unused_extern_crate)]
pub struct UnusedExternCrate {
    #[suggestion(applicability = "machine-applicable", code = "")]
    pub span: Span,
}

#[derive(LintDiagnostic)]
#[diag(typeck::extern_crate_not_idiomatic)]
pub struct ExternCrateNotIdiomatic {
    #[suggestion_short(applicability = "machine-applicable", code = "{suggestion_code}")]
    pub span: Span,
    pub msg_code: String,
    pub suggestion_code: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::safe_trait_implemented_as_unsafe, code = "E0199")]
pub struct SafeTraitImplementedAsUnsafe {
    #[primary_span]
    pub span: Span,
    pub trait_name: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::unsafe_trait_implemented_without_unsafe_keyword, code = "E0200")]
pub struct UnsafeTraitImplementedWithoutUnsafeKeyword {
    #[primary_span]
    pub span: Span,
    pub trait_name: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::attribute_requires_unsafe_keyword, code = "E0569")]
pub struct AttributeRequiresUnsafeKeyword<'a> {
    #[primary_span]
    pub span: Span,
    pub attr_name: &'a str,
}

#[derive(Eq, PartialEq)]
pub enum UnconstrainedParameterType {
    Type,
    Lifetime,
    Const,
}

pub struct TypeParameterNotConstrainedForImpl {
    pub span: Span,
    pub kind: UnconstrainedParameterType,
    pub name: Symbol,
}

impl<'a> SessionDiagnostic<'a> for TypeParameterNotConstrainedForImpl {
    fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
        let kind_msg = match self.kind {
            UnconstrainedParameterType::Type => "type",
            UnconstrainedParameterType::Lifetime => "lifetime",
            UnconstrainedParameterType::Const => "const",
        };

        let mut err = sess.span_diagnostic.struct_span_err_with_code(
            self.span,
            rustc_errors::fluent::typeck::type_parameter_not_constrained_for_impl,
            error_code!(E0207),
        );

        err.set_arg("name", self.name);
        err.set_arg("kind", kind_msg);

        err.span_label(self.span, rustc_errors::fluent::typeck::label);

        if self.kind == UnconstrainedParameterType::Const {
            err.note(rustc_errors::fluent::typeck::first_note);
            err.note(rustc_errors::fluent::typeck::second_note);
        }

        err
    }
}

#[derive(SessionDiagnostic)]
#[diag(typeck::associated_items_not_distinct, code = "E0201")]
pub struct AssociatedItemsNotDistinct {
    #[primary_span]
    #[label]
    pub span: Span,
    pub ident: String,
    #[label(typeck::prev_def_label)]
    pub prev_definition_span: Span,
}

#[derive(SessionSubdiagnostic)]
pub enum AssociatedTypeNotDefinedInTraitComment {
    #[suggestion(
        typeck::suggest_similarily_named_type,
        code = "{similar}",
        applicability = "maybe-incorrect"
    )]
    SuggestSimilarType {
        #[primary_span]
        span: Span,
        similar: Symbol,
    },
    #[label(typeck::label_similarily_named_type)]
    LabelSimilarType {
        #[primary_span]
        span: Span,
        suggested_name: Symbol,
        trait_name: String,
    },
    #[label(typeck::label_type_not_found)]
    CommentNotFound {
        #[primary_span]
        span: Span,
        assoc_name: Ident,
    },
}

#[derive(SessionDiagnostic)]
#[diag(typeck::associated_items_not_defined_in_trait, code = "E0220")]
pub struct AssociatedTypeNotDefinedInTrait<'a> {
    #[primary_span]
    pub span: Span,
    pub assoc_name: Ident,
    pub ty_param_name: &'a str,
    #[subdiagnostic]
    pub comment: AssociatedTypeNotDefinedInTraitComment,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::enum_discriminant_overflow, code = "E0370")]
pub struct EnumDiscriminantOverflow {
    #[primary_span]
    #[label]
    pub span: Span,
    pub last_good_discriminant: String,
    pub overflown_discriminant: Ident,
    pub wrapped_value: String,
    #[note]
    pub _note: (),
}

#[derive(SessionDiagnostic)]
#[diag(typeck::rustc_paren_sugar_not_enabled)]
pub struct RustcParenSugarNotEnabled {
    #[primary_span]
    pub span: Span,
    #[help]
    pub _help: (),
}

pub struct AttributeOnNonForeignFunction<'a> {
    pub span: Span,
    pub error_code: DiagnosticId,
    pub attr_name: &'a str,
}

// Manual implementation of `SessionDiagnostic` to be able to call `span_to_snippet`.
impl<'a, 'b> SessionDiagnostic<'a> for AttributeOnNonForeignFunction<'b> {
    fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
        let mut err = sess.span_diagnostic.struct_span_err_with_code(
            self.span,
            rustc_errors::fluent::typeck::attribute_on_non_foreign_function,
            self.error_code,
        );
        err.set_arg("attr_name", self.attr_name);

        err
    }
}

#[derive(SessionDiagnostic)]
#[diag(typeck::ffi_const_and_ffi_pure_on_same_function, code = "E0757")]
pub struct FFIConstAndFFIPureOnSameFunction {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::cmse_nonsecure_entry_requires_c_abi, code = "E0776")]
pub struct CMSENonSecureEntryRequiresCAbi {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::cmse_nonsecure_entry_requires_trust_zone_m_ext, code = "E0775")]
pub struct CMSENonSecureEntryRequiresTrustZoneMExt {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::track_caller_requires_cabi, code = "E0737")]
pub struct TrackCallerRequiresCAbi {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::export_name_contains_null_characters, code = "E0648")]
pub struct ExportNameContainsNullCharacters {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::instruction_set_unsupported_on_target, code = "E0779")]
pub struct InstructionSetUnsupportedOnTarget {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::varargs_on_non_cabi_function, code = "E0045")]
pub struct VarargsOnNonCabiFunction {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::generic_params_on_main_function, code = "E0131")]
pub struct GenericParamsOnMainFunction {
    #[primary_span]
    pub span: Span,
    #[label]
    pub generics_param_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::when_clause_on_main_function, code = "E0646")]
pub struct WhenClauseOnMainFunction {
    #[primary_span]
    pub span: Span,
    #[label]
    pub generics_where_clauses_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::async_main_function, code = "E0752")]
pub struct AsyncMainFunction {
    #[primary_span]
    pub span: Span,
    #[label]
    pub asyncness_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::generic_return_type_on_main, code = "E0131")]
pub struct GenericReturnTypeOnMain {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::type_parameter_on_start_function, code = "E0132")]
pub struct TypeParameterOnStartFunction {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::where_clause_on_start_function, code = "E0647")]
pub struct WhereClauseOnStartFunction {
    #[primary_span]
    pub span: Span,
    #[label]
    pub where_clause_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::async_start_function, code = "E0752")]
pub struct AsyncStartFunction {
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionSubdiagnostic)]
pub enum AmbiguousAssociatedTypeFixSuggestion<'a> {
    #[suggestion(
        typeck::fix_std_module_text,
        code = "std::",
        applicability = "machine-applicable"
    )]
    StdModule {
        #[primary_span]
        span: Span,
    },
    #[suggestion(
        typeck::fix_use_fully_qualified_syntax,
        code = "<{type_str} as {trait_str}>::{name}",
        applicability = "has-placeholders"
    )]
    UseFullyQualifiedSyntax {
        #[primary_span]
        span: Span,
        type_str: &'a str,
        trait_str: &'a str,
        name: Symbol,
    },
}

#[derive(SessionDiagnostic)]
#[diag(typeck::ambiguous_associated_type, code = "E0223")]
pub struct AmbiguousAssociatedType<'a> {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub possible_fix: AmbiguousAssociatedTypeFixSuggestion<'a>,
}

#[derive(SessionSubdiagnostic)]
pub enum EnumVariantNotFoundFixOrInfo<'a> {
    #[suggestion(
        typeck::fix_similar_type,
        code = "{suggested_name}",
        applicability = "maybe-incorrect"
    )]
    SuggestSimilarName {
        #[primary_span]
        span: Span,
        suggested_name: Symbol,
    },
    #[label(typeck::info_label)]
    InfoLabel {
        #[primary_span]
        span: Span,
        self_type: &'a str,
    },
}

#[derive(SessionDiagnostic)]
#[diag(typeck::enum_variant_not_found, code = "E0599")]
pub struct EnumVariantNotFound<'a> {
    #[primary_span]
    pub span: Span,
    #[label(typeck::info_label_at_enum)]
    pub info_label_at_enum: Option<Span>,
    #[subdiagnostic]
    pub fix_or_info: EnumVariantNotFoundFixOrInfo<'a>,
    pub assoc_ident: Ident,
    pub self_type: &'a str,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::expected_used_symbol)]
pub struct ExpectedUsedSymbol {
    #[primary_span]
    pub span: Span,
}

pub enum InvalidDispatchFromDynDeclarationType {
    TypesDifferTooMuch { source_path: String, target_path: String },
    InvalidRepr,
    InvalidFields { field_name: Symbol, ty_a: String },
    NoCoercedFields,
    TooManyCoercedFields { coerced_fields_len: usize, coerced_fields: String },
    NotAStruct,
}

pub struct InvalidDispatchFromDynDeclaration {
    pub(crate) span: Span,
    pub(crate) err_type: InvalidDispatchFromDynDeclarationType,
}

// Manual implementation of `SessionDiagnostic` to be able to call `span_to_snippet`.
impl<'a> SessionDiagnostic<'a> for InvalidDispatchFromDynDeclaration {
    fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
        let msg = match &self.err_type {
            InvalidDispatchFromDynDeclarationType::TypesDifferTooMuch { .. } => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_types_differ_too_much
            }
            InvalidDispatchFromDynDeclarationType::InvalidRepr => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_invalid_repr
            }
            InvalidDispatchFromDynDeclarationType::InvalidFields { .. } => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_invalid_fields
            }
            InvalidDispatchFromDynDeclarationType::NoCoercedFields => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_no_coerced_fields
            }
            InvalidDispatchFromDynDeclarationType::TooManyCoercedFields { .. } => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_too_many_coerced_fields
            }
            InvalidDispatchFromDynDeclarationType::NotAStruct => {
                rustc_errors::fluent::typeck::invalid_dispatch_from_dyn_not_a_struct
            }
        };

        let mut err =
            sess.span_diagnostic.struct_span_err_with_code(self.span, msg, error_code!(E0378));

        match self.err_type {
            InvalidDispatchFromDynDeclarationType::TypesDifferTooMuch {
                source_path,
                target_path,
            } => {
                err.set_arg("source_path", source_path);
                err.set_arg("target_path", target_path);
            }
            InvalidDispatchFromDynDeclarationType::InvalidFields { field_name, ty_a } => {
                err.set_arg("field_name", field_name);
                err.set_arg("ty_a", ty_a);

                err.note(rustc_errors::fluent::typeck::note);
            }
            InvalidDispatchFromDynDeclarationType::TooManyCoercedFields {
                coerced_fields_len,
                coerced_fields,
            } => {
                err.set_arg("coerced_fields_len", coerced_fields_len);
                err.set_arg("coerced_fields", coerced_fields);

                err.note(rustc_errors::fluent::typeck::note);
                err.note(rustc_errors::fluent::typeck::fields_that_need_coercions_fields);
            }
            _ => {}
        }

        err
    }
}

#[derive(SessionDiagnostic)]
#[diag(typeck::coerce_unsized_invalid_definition, code = "E0377")]
pub struct CoerceUnsizedInvalidDefinition {
    #[primary_span]
    pub span: Span,
    pub source_path: String,
    pub target_path: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::coerce_unsized_no_coerced_field, code = "E0374")]
pub struct CoerceUnsizedNoCoercedField {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::coerce_unsized_no_coerced_field, code = "E0375")]
pub struct CoerceUnsizedTooManyCoercedFields {
    #[primary_span]
    #[label]
    pub span: Span,
    #[note]
    pub _note: (),
    #[note(typeck::fields_that_need_coercions_fields)]
    pub _fields_note: (),
    pub coerced_fields_len: usize,
    pub coerced_fields: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::coerce_unsized_not_a_struct, code = "E0376")]
pub struct CoerceUnsizedNotAStruct {
    #[primary_span]
    pub span: Span,
}

pub struct ExplicitImplOfInternalStructs {
    pub span: Span,
    pub error_code: DiagnosticId,
    pub trait_name: &'static str,
}

impl<'a> SessionDiagnostic<'a> for ExplicitImplOfInternalStructs {
    fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
        let mut err = sess.span_diagnostic.struct_span_err_with_code(
            self.span,
            rustc_errors::fluent::typeck::explicit_impl_of_internal_structs,
            self.error_code,
        );

        err.set_arg("trait_name", self.trait_name);

        err.span_label(self.span, rustc_errors::fluent::typeck::label);

        err
    }
}

#[derive(SessionDiagnostic)]
#[diag(typeck::marker_trait_impl_contains_items, code = "E0715")]
pub struct MarkerTraitImplContainsItems {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::type_automatically_implements_trait, code = "E0371")]
pub struct TypeAutomaticallyImplementsTrait {
    #[primary_span]
    #[label]
    pub span: Span,
    pub object_type: String,
    pub trait_path: String,
}

#[derive(SessionDiagnostic)]
#[diag(typeck::cross_crate_opt_out_trait_impl_on_invalid_target, code = "E0321")]
pub struct CrossCrateOptOutTraitImplOnInvalidTarget {
    #[primary_span]
    #[label]
    pub span: Span,
    pub trait_path: String,
    pub error_type: &'static str,
    pub self_type: String,
}
