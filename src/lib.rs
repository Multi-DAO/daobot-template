use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{AccountId, env, near_bindgen, ext_contract, log};
use serde::Deserialize;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Daobot {
    nft_id: AccountId,
    dao_id: AccountId,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum ProposalKinds {
    ChangeConfig {  },
    ChangePolicy {  },
    ChangePolicyAddOrUpdateRole {  },
    ChangePolicyRemoveRole {  },
    ChangePolicyUpdateDefaultVotePolicy {  },
    ChangePolicyUpdateParameters {  },
    AddMemberToRole { member_id: AccountId },
    RemoveMemberFromRole {  },
    FunctionCall {  },
    UpgradeSelf {  },
    UpgradeRemote {  },
    Transfer {  },
    SetStakingContract {  },
    AddBounty {  },
    BountyDone {  },
    Vote,
    FactoryInfoUpdate {  },
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    id: u64,
    kind: ProposalKinds,
}

#[ext_contract(ext_astrodao)]
pub trait Astrodao {
     fn version(&self) -> String;
     fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<Proposal>;
    fn get_proposal(&self, id: u64) -> Proposal;
     fn act_proposal(&self, id: u64, action: String);
     fn get_last_proposal_id(&self) -> u64;
}

#[ext_contract(ext_nft_enum)]
pub trait NftEnumOwners {
    fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128;
}



// Recieve callbacks from external contract.
#[ext_contract(ext_self)]
trait Callbacks {
    // fn on_get_proposals(&self, #[callback] proposals: Vec<Proposal>);
    fn on_get_proposal(&self, #[callback] proposal: Proposal);
    fn on_get_last_proposal_id(&self, #[callback] last_proposal_id: u64);
    fn on_approve_proposal(&self, proposal_id: u64, #[callback] nft_supply: U128);
    fn proposal_approved(&self, id: u64);
    fn proposal_rejected(&self, id: u64);
}   

impl Default for Daobot {
    fn default() -> Self {
        Self {
            nft_id: AccountId::from(""),
            dao_id: AccountId::from(""),
        }
    }
}
pub const GAS_FOR_DAO_VIEW: u64 = 6_000_000_000_000;
pub const GAS_FOR_DAO_CALL: u64 = 10_000_000_000_000;
pub const GAS_MARGIN: u64 = 12_000_000_000_000;

#[near_bindgen]
impl Daobot {
    pub fn approve_members(&mut self, dao_id: AccountId, nft_id: AccountId)  {
        self.nft_id = nft_id;
        self.dao_id = dao_id;
        log!("before x-contract in approve_members prepaid_gas {}, used_gas: {}", env::prepaid_gas(), env::used_gas());
        let callback = ext_self::on_get_last_proposal_id (&env::current_account_id(), 0, env::prepaid_gas() - (env::used_gas() + GAS_FOR_DAO_VIEW + GAS_MARGIN));
        ext_astrodao::get_last_proposal_id(&self.dao_id,0, GAS_FOR_DAO_VIEW).then(callback);
        log!("contract call scheduled");
    }

    #[private]
    pub fn on_get_last_proposal_id(&self, #[callback] last_proposal_id: u64) {
        log!("In get_last_proposal_id");
        // let callback = ext_self::on_get_proposals(dao_id.clone(),&env::current_account_id(), 0, gas_per_call);
        let mono_callback = ext_self::on_get_proposal(&env::current_account_id(), 0, env::prepaid_gas() - env::used_gas()- GAS_FOR_DAO_VIEW - GAS_MARGIN);
        ext_astrodao::get_proposal(last_proposal_id-1, &self.dao_id, 0, GAS_FOR_DAO_VIEW).then(mono_callback);
        // ext_astrodao::get_proposals(max(100,last_proposal_id)-100, 100, &dao_id, 0, gas_per_call*2 )
        // .then(callback);
    }

    // #[private]
    // pub fn on_get_proposals(&self, #[callback] proposals: Vec<Proposal>)  {
        
    //     let mut active_proposals = proposals.iter().filter(|p| p.status == "InProgress".to_string() && match p.kind {
    //         ProposalKinds::AddMemberToRole { member_id: _ } => true,
    //         _ => false,
    //     } ).peekable();
        
    //     if active_proposals.peek().is_none() {
    //         panic!("No active proposals");
    //     }
    //     let proposal_count = (active_proposals.clone().count()) as u64;
        
    //     const ESTIMATED_USED_GAS: u64 = 7e12 as u64;
    //     let remaining_gas = env::prepaid_gas() - env::used_gas() - ESTIMATED_USED_GAS;
    //     // 2 calls plus one extra for overhead
    //     const CALLS_PER_LOOP: u64 = 4;
    //     let gas_per_call = remaining_gas / (proposal_count * CALLS_PER_LOOP);
    //     log!("Gas per call: {:?}", gas_per_call);
    //     active_proposals.for_each(|p| {  
    //         log!("Remaining gas: {:?}", env::prepaid_gas() - env::used_gas());
    //         let callback = ext_self::on_approve_proposal(p.id, &env::current_account_id(), 0, gas_per_call);
    //         ext_nft_enum::nft_supply_for_owner(match &p.kind {
    //             ProposalKinds::AddMemberToRole { member_id } => member_id.to_owned(),
    //             _ => panic!("Unexpected proposal kind"),
    //         }, &self.nft_id, 0, gas_per_call).then(callback);

    //     });
    // }

    #[private]
    pub fn on_approve_proposal(&self, proposal_id: u64, #[callback] nft_supply: U128) {
        if nft_supply.0 > 0 {
            let approved = ext_self::proposal_approved(proposal_id, &env::current_account_id(),0, env::prepaid_gas() - env::used_gas() - GAS_FOR_DAO_CALL - GAS_MARGIN);
            ext_astrodao::act_proposal(proposal_id, "VoteApprove".to_string(), &self.dao_id, 0, GAS_FOR_DAO_CALL).then(approved);
        } else {
            let rejected = ext_self::proposal_rejected(proposal_id, &env::current_account_id(),0, env::prepaid_gas() - env::used_gas() - GAS_FOR_DAO_CALL - GAS_MARGIN);
            ext_astrodao::act_proposal(proposal_id, "VoteReject".to_string(), &self.dao_id, 0, GAS_FOR_DAO_CALL).then(rejected);
        }
}

    #[private]
    pub fn on_get_proposal(&self, #[callback] proposal: Proposal)  {

        let proposal_id = proposal.id;
        let user_id = match proposal.kind {
            ProposalKinds::AddMemberToRole { member_id } => member_id.to_owned(),
            _ => panic!("Unexpected proposal kind"),
        };

        let callback = ext_self::on_approve_proposal(proposal_id, &env::current_account_id(), 0, env::prepaid_gas() - env::used_gas() - GAS_FOR_DAO_VIEW - GAS_MARGIN);
        ext_nft_enum::nft_supply_for_owner(user_id, &self.nft_id, 0, GAS_FOR_DAO_VIEW).then(callback);

    }

    #[private]
    pub fn proposal_approved(&self, id: u64) {
        log!("Proposal {} approved", id);

    }

    #[private]
    pub fn proposal_rejected(&self, id: u64) {
        log!("Proposal {} rejected", id);
    }
}
