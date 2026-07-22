//! [GAP4-R2-PILOT] Thin executable pilot of the SPEC §4.2 unified label
//! lattice (DP4 — the ONE novel type-system concept). Scope: lattice math +
//! sink-bound mechanics only. This is NOT full IFC, NOT the CONF-P2
//! label-inference ergonomics gate, and implicit flows stay [UNVERIFIED/OPEN]
//! per SPEC §4.2. No language-surface integration yet.

use std::collections::BTreeSet;

/// One label atom: authority (capability name) or data (provenance/secrecy).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    /// Authority atom — a capability name (MVP: "console"; more post-MVP).
    Auth(String),
    /// Data atom: content may be attacker-influenced (integrity).
    Untrusted,
    /// Data atom: content must not reach public sinks (confidentiality).
    Secret,
}

/// A label is a finite set of atoms, ordered by set inclusion (SPEC §4.2):
/// `L1 ⊑ L2 iff L1 ⊆ L2`; join = ∪, meet = ∩, ⊥ = ∅. "Lower is better".
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Label(pub BTreeSet<Atom>);

impl Label {
    /// ⊥ — pure computation, trusted public data.
    pub fn bottom() -> Self {
        Label(BTreeSet::new())
    }

    pub fn of(atoms: &[Atom]) -> Self {
        Label(atoms.iter().cloned().collect())
    }

    /// Lattice join (∪).
    pub fn join(&self, other: &Self) -> Self {
        Label(self.0.union(&other.0).cloned().collect())
    }

    /// Lattice meet (∩).
    pub fn meet(&self, other: &Self) -> Self {
        Label(self.0.intersection(&other.0).cloned().collect())
    }

    /// Lattice order: `self ⊑ other` iff subset.
    pub fn leq(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    /// (SUB-LABEL) covariance: `T^{L1} <: T^{L2}` iff `L1 ⊆ L2`. A parameter's
    /// declared label is an UPPER BOUND on what it accepts — so a ∅-data sink
    /// bound rejects `untrusted` (E1 injection) and `secret` (E6 leak) values.
    pub fn flows_to(&self, bound: &Self) -> bool {
        self.leq(bound)
    }

    /// The data-atom projection of this label.
    pub fn data(&self) -> Self {
        Label(
            self.0
                .iter()
                .filter(|a| matches!(a, Atom::Untrusted | Atom::Secret))
                .cloned()
                .collect(),
        )
    }

    /// (TAINT-PROP) computation joins DATA atoms only — authority atoms ride
    /// closures/handles, they do not taint results.
    pub fn taint_prop(&self, other: &Self) -> Self {
        self.data().join(&other.data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u() -> Label {
        Label::of(&[Atom::Untrusted])
    }
    fn s() -> Label {
        Label::of(&[Atom::Secret])
    }
    fn con() -> Label {
        Label::of(&[Atom::Auth("console".into())])
    }

    #[test]
    fn gap4_lattice_laws_hold() {
        let us = u().join(&s());
        // commutativity / idempotence / identity / order coherence
        assert_eq!(u().join(&s()), s().join(&u()));
        assert_eq!(u().join(&u()), u());
        assert_eq!(u().join(&Label::bottom()), u());
        assert_eq!(us.meet(&u()), u());
        assert!(Label::bottom().leq(&u()) && u().leq(&us) && !us.leq(&u()));
    }

    #[test]
    fn gap4_sub_label_sink_bounds() {
        // E1 shape: db.insert(row: User) bounds at ∅-data -> untrusted rejected.
        assert!(!u().flows_to(&Label::bottom()));
        // E6 shape: console.print(s: Str) bounds at ∅-data -> secret rejected.
        assert!(!s().flows_to(&Label::bottom()));
        // Trusted public data flows anywhere; a value flows to a wider bound.
        assert!(Label::bottom().flows_to(&u()));
        assert!(u().flows_to(&u().join(&s())));
        // A sink typed to accept secrets does accept them (net.connect(auth:)).
        assert!(s().flows_to(&s()));
    }

    #[test]
    fn gap4_taint_prop_joins_data_only() {
        // A console-capability handle in the computation does not taint data;
        // untrusted + secret operands taint the result with both data atoms.
        assert_eq!(con().taint_prop(&u()), u());
        assert_eq!(u().taint_prop(&s()), u().join(&s()));
        assert_eq!(con().taint_prop(&con()), Label::bottom());
    }
}
