#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_smir;
extern crate stable_mir;

use rustc_driver::{Callbacks, Compilation, RunCompiler};
use rustc_interface::{interface, Queries};
use rustc_middle::ty::TyCtxt;
use rustc_session::EarlyErrorHandler;
use rustc_smir::rustc_internal;
use std::collections::BTreeSet;
use std::env;
use std::ops::ControlFlow;

use formality_prove::{TraitDecl, TraitDeclBoundData};
use formality_smir::ToFormality;
use formality_types::derive_links::UpcastFrom;
use formality_types::grammar::{fresh_bound_var, Binder, TraitId, Wc, Wcs};
// next version
//use rustc_smir::run;

const CRATE_NAME: &str = "smir_formality";

/// Run the compiler using the given filename, registering a Stable MIR processing callback right
/// after analysis that generates Stable MIR of the given rust file and then converts that Stable
/// MIR into Formality.
fn main() {
    let args: Vec<String> = env::args().collect();
    let args = vec![
        "rustc +nightly".to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        args[1].clone(),
    ];
    //next version ...
    //run!(args, tcx, smir_formality(tcx)).unwrap();
    rustc_internal::StableMir::new(args, smir_formality)
        .run()
        .unwrap();
}

/// This function uses the Stable MIR APIs to get generate Stable MIR representation of the given
/// program and converts that into Formality.
fn smir_formality(tcx: TyCtxt<'_>) -> ControlFlow<()> {
    let trait_decls: Vec<_> = stable_mir::all_trait_decls()
        .iter()
        .map(|trait_def| {
            let trait_decl = stable_mir::trait_decl(trait_def);
            let generics = trait_decl.generics_of();
            let predicates = trait_decl.explicit_predicates_of().predicates;

            println!("{:?}", trait_decl);
            generics.params.iter().for_each(|generic| {
                println!("{:?}", generic);
            });
            predicates.iter().for_each(|predicate| {
                println!("{:?}", predicate);
            });

            TraitDecl {
                id: TraitId::new(&format!("{:?}", trait_decl.def_id)),
                binder: Binder::new(
                    generics
                        .params
                        .iter()
                        .map(|param| {
                            let param_kind = param.kind.formality();
                            fresh_bound_var(param_kind)
                        })
                        .collect::<Vec<_>>(),
                    TraitDeclBoundData {
                        where_clause: Wcs::from_iter(
                            predicates
                                .iter()
                                .map(|(predicate, _)| Wc::upcast_from(predicate.formality())),
                        ),
                    },
                ),
            }
        })
        .collect();

    println!("------- formality ------------");
    println!("{:?}", trait_decls);

    stable_mir::all_trait_impls()
        .iter()
        .for_each(|impl_trait_decl| {
            println!("{:?}", impl_trait_decl);
            let impl_trait = stable_mir::trait_impl(impl_trait_decl);
            let impl_trait = impl_trait.value;
            println!("impl_trait.def_id = {:?}", impl_trait.def_id);
            println!("impl_trait.args = {:?}", impl_trait.args);
        });

    ControlFlow::Continue(())
}
