use scrypto::engine::types::*;
use scrypto::rust::cell::RefCell;
use scrypto::rust::collections::BTreeSet;
use scrypto::rust::collections::HashMap;
use scrypto::rust::rc::Rc;
use scrypto::rust::vec::Vec;

use crate::model::{
    LockedAmountOrIds, ResourceContainer, ResourceContainerError, ResourceContainerId,
};

#[derive(Debug)]
pub struct Proof {
    /// The resource definition id.
    resource_def_id: ResourceDefId,
    /// The resource type.
    resource_type: ResourceType,
    /// Whether movement of this proof is restricted.
    restricted: bool,
    /// The total locked amount or non-fungible ids.
    total_locked: LockedAmountOrIds,
    /// The supporting containers.
    evidence: HashMap<ResourceContainerId, (Rc<RefCell<ResourceContainer>>, LockedAmountOrIds)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofError {
    /// Error produced by a resource container.
    ResourceContainerError(ResourceContainerError),
    /// Can't generate zero-amount or empty non-fungible set proofs.
    EmptyProofNotAllowed,
    /// The base proofs are not enough to cover the requested amount or non-fungible ids.
    InsufficientBaseProofs,
    /// Can't apply a non-fungible operation on fungible proofs.
    NonFungibleOperationNotAllowed,
    /// Can't apply a fungible operation on non-fungible proofs.
    FungibleOperationNotAllowed,
}

impl Proof {
    pub fn new(
        resource_def_id: ResourceDefId,
        resource_type: ResourceType,
        restricted: bool,
        total_locked: LockedAmountOrIds,
        evidence: HashMap<ResourceContainerId, (Rc<RefCell<ResourceContainer>>, LockedAmountOrIds)>,
    ) -> Result<Proof, ProofError> {
        if total_locked.is_empty() {
            return Err(ProofError::EmptyProofNotAllowed);
        }

        Ok(Self {
            resource_def_id,
            resource_type,
            restricted,
            total_locked,
            evidence,
        })
    }

    /// Computes the locked amount or non-fungible IDs, in total and per resource container.
    pub fn compute_total_locked(
        proofs: &[Proof],
        resource_def_id: ResourceDefId,
        resource_type: ResourceType,
    ) -> (
        LockedAmountOrIds,
        HashMap<ResourceContainerId, LockedAmountOrIds>,
    ) {
        // filter proofs by resource def id and restricted flag
        let proofs: Vec<&Proof> = proofs
            .iter()
            .filter(|p| p.resource_def_id() == resource_def_id && !p.is_restricted())
            .collect();

        // calculate the max locked amount (or ids) of each container
        match resource_type {
            ResourceType::Fungible { .. } => {
                let mut max = HashMap::<ResourceContainerId, Decimal>::new();
                for proof in &proofs {
                    for (container_id, (_, locked_amount_or_ids)) in &proof.evidence {
                        let new_amount = locked_amount_or_ids.amount();
                        if let Some(existing) = max.get_mut(&container_id) {
                            *existing = Decimal::max(*existing, new_amount);
                        } else {
                            max.insert(container_id.clone(), new_amount);
                        }
                    }
                }
                let total = max
                    .values()
                    .cloned()
                    .reduce(|a, b| a + b)
                    .unwrap_or_default();
                let per_container = max
                    .into_iter()
                    .map(|(k, v)| (k, LockedAmountOrIds::Amount(v)))
                    .collect();
                (LockedAmountOrIds::Amount(total), per_container)
            }
            ResourceType::NonFungible => {
                let mut max = HashMap::<ResourceContainerId, BTreeSet<NonFungibleId>>::new();
                for proof in &proofs {
                    for (container_id, (_, locked_amount_or_ids)) in &proof.evidence {
                        let new_ids = locked_amount_or_ids.ids().unwrap();
                        if let Some(ids) = max.get_mut(&container_id) {
                            ids.extend(new_ids);
                        } else {
                            max.insert(container_id.clone(), new_ids);
                        }
                    }
                }
                let mut total = BTreeSet::<NonFungibleId>::new();
                for value in max.values() {
                    total.extend(value.clone());
                }
                let per_container = max
                    .into_iter()
                    .map(|(k, v)| (k, LockedAmountOrIds::Ids(v)))
                    .collect();
                (LockedAmountOrIds::Ids(total), per_container)
            }
        }
    }

    /// Creates a composite proof from proofs. This method will generate a max proof.
    pub fn compose(
        proofs: &[Proof],
        resource_def_id: ResourceDefId,
        resource_type: ResourceType,
    ) -> Result<Proof, ProofError> {
        let (total, _) = Self::compute_total_locked(proofs, resource_def_id, resource_type);
        match total {
            LockedAmountOrIds::Amount(amount) => {
                Self::compose_by_amount(proofs, amount, resource_def_id, resource_type)
            }
            LockedAmountOrIds::Ids(ids) => {
                Self::compose_by_ids(proofs, &ids, resource_def_id, resource_type)
            }
        }
    }

    pub fn compose_by_amount(
        proofs: &[Proof],
        amount: Decimal,
        resource_def_id: ResourceDefId,
        resource_type: ResourceType,
    ) -> Result<Proof, ProofError> {
        todo!("Re-implement")
    }

    pub fn compose_by_ids(
        proofs: &[Proof],
        ids: &BTreeSet<NonFungibleId>,
        resource_def_id: ResourceDefId,
        resource_type: ResourceType,
    ) -> Result<Proof, ProofError> {
        todo!("Re-implement")
    }

    /// Makes a clone of this proof.
    ///
    /// Note that cloning a proof will update the ref count of the locked
    /// resources in the source containers.
    pub fn clone(&self) -> Self {
        for (_, (container, evidence)) in &self.evidence {
            match evidence {
                LockedAmountOrIds::Amount(amount) => {
                    container
                        .borrow_mut()
                        .lock_by_amount(*amount)
                        .expect("Cloning should always succeed");
                }
                LockedAmountOrIds::Ids(ids) => {
                    container
                        .borrow_mut()
                        .lock_by_ids(ids)
                        .expect("Cloning should always succeed");
                }
            }
        }
        Self {
            resource_def_id: self.resource_def_id.clone(),
            resource_type: self.resource_type.clone(),
            restricted: self.restricted,
            total_locked: self.total_locked.clone(),
            evidence: self.evidence.clone(),
        }
    }

    pub fn drop(self) {
        for (_, (container, evidence)) in self.evidence {
            container.borrow_mut().unlock(evidence);
        }
    }

    pub fn change_to_restricted(&mut self) {
        self.restricted = true;
    }

    pub fn resource_def_id(&self) -> ResourceDefId {
        self.resource_def_id
    }

    pub fn total_amount(&self) -> Decimal {
        self.total_locked.amount()
    }

    pub fn total_ids(&self) -> Result<BTreeSet<NonFungibleId>, ProofError> {
        self.total_locked
            .ids()
            .map_err(|_| ProofError::NonFungibleOperationNotAllowed)
    }

    pub fn is_restricted(&self) -> bool {
        self.restricted
    }
}
