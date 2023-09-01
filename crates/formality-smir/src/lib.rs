#![feature(rustc_private)]

/// This import is needed, because `stable_mir` on its own doesn't have the `scoped_tls` rlib.
extern crate rustc_driver;
/// Access to the pre-0.1 stable_mir crate
extern crate stable_mir;

use formality_types::cast::Upcast;
use formality_types::grammar::{fresh_bound_var, Binder, Predicate, TraitId, TraitRef, TyData};

/// Trait used to convert from Stable MIR to Formality types.
pub trait ToFormality {
    /// The formality representation of the stable MIR type implementing ToFormality.
    type T;
    /// Converts an object to the equivalent Formality representation.
    fn formality(&self) -> Self::T;
}

impl ToFormality for stable_mir::ty::GenericParamDefKind {
    type T = formality_types::derive_links::ParameterKind;

    fn formality(&self) -> Self::T {
        use formality_types::derive_links::ParameterKind;

        match self {
            stable_mir::ty::GenericParamDefKind::Lifetime => ParameterKind::Lt,
            stable_mir::ty::GenericParamDefKind::Type { .. } => ParameterKind::Ty,
            stable_mir::ty::GenericParamDefKind::Const { .. } => ParameterKind::Const,
        }
    }
}

impl ToFormality for stable_mir::ty::GenericArgKind {
    type T = formality_types::derive_links::Parameter;

    fn formality(&self) -> Self::T {
        use formality_types::derive_links::Parameter;

        match self {
            stable_mir::ty::GenericArgKind::Lifetime(_lt) => {
                todo!()
                // Parameter::Lt(lt.formality()),
            }
            stable_mir::ty::GenericArgKind::Type(ty) => Parameter::Ty(ty.formality()),
            stable_mir::ty::GenericArgKind::Const(_c) => {
                todo!()
                // Parameter::Const(c.formality()),
            }
        }
    }
}

impl ToFormality for stable_mir::ty::Ty {
    type T = formality_types::grammar::Ty;

    fn formality(&self) -> Self::T {
        use stable_mir::ty::TyKind;

        match self.kind() {
            TyKind::RigidTy(rigid_ty) => {
                // TyData::RigidTy(rigid_ty)
                todo!();
            }
            TyKind::Alias(alias_kind, alias_ty) => {
                // TyData::AliasTy(alias_ty)
                todo!();
            }
            TyKind::Param(param_ty) => TyData::PredicateTy(param_ty.formality()).upcast(),
            TyKind::Bound(u, bound_ty) => {
                // TyData::Variable(variable)
                todo!();
            }
        }
    }
}

impl ToFormality for stable_mir::ty::ParamTy {
    type T = formality_types::grammar::PredicateTy;

    fn formality(&self) -> Self::T {
        use formality_types::derive_links::ParameterKind;
        use formality_types::derive_links::Variable;
        use formality_types::grammar::PredicateTy;

        let bound_var = fresh_bound_var(ParameterKind::Ty);

        PredicateTy::ForAll(Binder::new(
            // map to the right index on the generic_arg list using index to get the kind and
            // also the correspoding index for bound, is not a fresh_bound_var
            vec![bound_var],
            // this is wrong
            TyData::Variable(Variable::BoundVar(bound_var)).upcast(),
        ))
    }
}

impl ToFormality for stable_mir::ty::PredicateKind {
    type T = formality_types::grammar::WcData;

    // pub enum WcData {
    //     #[cast]
    //     PR(PR),
    //
    //     #[grammar(for $v0)]
    //     ForAll(Binder<Wc>),
    //
    //     #[grammar(if $v0 $v1)]
    //     Implies(Wcs, Wc),
    // }

    fn formality(&self) -> Self::T {
        match self {
            stable_mir::ty::PredicateKind::Clause(clause_kind) => clause_kind.formality().upcast(),
            stable_mir::ty::PredicateKind::ObjectSafe(trait_def) => {
                todo!();
            }
            stable_mir::ty::PredicateKind::ClosureKind(closure_def, generic_arg, closure_kind) => {
                todo!();
            }
            stable_mir::ty::PredicateKind::SubType(subtype_predicate) => {
                todo!();
            }
            stable_mir::ty::PredicateKind::Coerce(coerce_predicate) => {
                todo!();
            }
            stable_mir::ty::PredicateKind::ConstEquate(const_a, const_b) => {
                todo!();
            }
            stable_mir::ty::PredicateKind::Ambiguous => {
                todo!();
            }
            stable_mir::ty::PredicateKind::AliasRelate(
                term_kind_a,
                term_kind_b,
                alias_relation_direction,
            ) => {
                todo!();
            }
        }
    }
}

impl ToFormality for stable_mir::ty::ClauseKind {
    type T = formality_types::grammar::PR;

    fn formality(&self) -> Self::T {
        match self {
            stable_mir::ty::ClauseKind::Trait(trait_predicate) => {
                let predicate: Predicate = TraitRef::new(
                    &TraitId::new(&format!("{:?}", trait_predicate.trait_ref.def_id)),
                    trait_predicate
                        .trait_ref
                        .args
                        .0
                        .iter()
                        .map(|param_kind| param_kind.formality())
                        .collect::<Vec<_>>(),
                )
                .upcast();
                predicate.upcast()
            }
            stable_mir::ty::ClauseKind::RegionOutlives(region_outlives_predicate) => {
                todo!();
            }
            stable_mir::ty::ClauseKind::TypeOutlives(type_outlives_predicate) => {
                todo!();
            }
            stable_mir::ty::ClauseKind::Projection(projection_predicate) => {
                todo!();
            }
            stable_mir::ty::ClauseKind::ConstArgHasType(const_, ty) => {
                todo!();
            }
            stable_mir::ty::ClauseKind::WellFormed(generic_arg_kind) => {
                todo!();
            }
            stable_mir::ty::ClauseKind::ConstEvaluatable(const_) => {
                todo!();
            }
        }
    }
}
