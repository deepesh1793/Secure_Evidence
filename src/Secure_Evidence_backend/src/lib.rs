use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::collections::HashMap;

type EvidenceId = u64;

#[derive(Clone, CandidType, Deserialize)]
struct Evidence {
    id: EvidenceId,
    owner: String,
    data: Vec<u8>,
    timestamp: u64,
}

#[derive(Default)]
struct EvidenceStore {
    evidence: HashMap<EvidenceId, Evidence>,
    next_id: EvidenceId,
}

thread_local! {
    static STORAGE: std::cell::RefCell<EvidenceStore> = std::cell::RefCell::new(EvidenceStore::default());
}

#[update]
fn submit_evidence(owner: String, data: Vec<u8>) -> Result<EvidenceId, String> {
    let caller = ic_cdk::api::caller().to_text();
    if caller != owner {
        return Err("Caller is not the owner".to_string());
    }

    let mut store = STORAGE.with(|store| store.borrow_mut());
    let id = store.next_id;
    let timestamp = ic_cdk::api::time();
    store.evidence.insert(
        id,
        Evidence {
            id,
            owner,
            data,
            timestamp,
        },
    );
    store.next_id += 1;
    Ok(id)
}

#[query]
fn get_evidence(id: EvidenceId) -> Option<Evidence> {
    let store = STORAGE.with(|store| store.borrow());
    store.evidence.get(&id).cloned()
}

#[update]
fn delete_evidence(id: EvidenceId) -> Result<(), String> {
    let caller = ic_cdk::api::caller().to_text();

    STORAGE.with(|store| {
        let mut store = store.borrow_mut();
        if let Some(evidence) = store.evidence.get(&id) {
            if evidence.owner != caller {
                return Err("Caller is not the owner".to_string());
            }
        } else {
            return Err("Evidence not found".to_string());
        }
        store.evidence.remove(&id);
        Ok(())
    })
}

#[export_name = "canister_query get_evidence"]
fn export_get_evidence() {
    ic_cdk::export::candid::export_service!();
    ic_cdk::api::print!("{}", export_service());
}

#[export_name = "canister_update submit_evidence"]
fn export_submit_evidence() {
    ic_cdk::export::candid::export_service!();
    ic_cdk::api::print!("{}", export_service());
}

#[export_name = "canister_update delete_evidence"]
fn export_delete_evidence() {
    ic_cdk::export::candid::export_service!();
    ic_cdk::api::print!("{}", export_service());
}

#[ic_cdk_macros::init]
fn init() {
    ic_cdk::setup();
}
