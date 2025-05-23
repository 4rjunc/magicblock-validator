use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use solana_sdk::{
    account::Account, bpf_loader_upgradeable::get_program_data_address,
    pubkey::Pubkey, signature::Signature,
};

use crate::{AccountDumper, AccountDumperResult};

#[derive(Debug, Clone, Default)]
pub struct AccountDumperStub {
    feepayer_accounts: Arc<RwLock<HashSet<Pubkey>>>,
    undelegated_accounts: Arc<RwLock<HashSet<Pubkey>>>,
    delegated_accounts: Arc<RwLock<HashSet<Pubkey>>>,
    program_ids: Arc<RwLock<HashSet<Pubkey>>>,
    program_datas: Arc<RwLock<HashSet<Pubkey>>>,
    program_idls: Arc<RwLock<HashSet<Pubkey>>>,
}

impl AccountDumper for AccountDumperStub {
    fn dump_feepayer_account(
        &self,
        pubkey: &Pubkey,
        _lamports: u64,
        _owner: &Pubkey,
    ) -> AccountDumperResult<Signature> {
        self.feepayer_accounts
            .write()
            .expect("RwLock for feepayer_accounts is poisoned")
            .insert(*pubkey);
        Ok(Signature::new_unique())
    }

    fn dump_undelegated_account(
        &self,
        pubkey: &Pubkey,
        _account: &Account,
    ) -> AccountDumperResult<Signature> {
        self.undelegated_accounts
            .write()
            .expect("RwLock for undelegated_accounts is poisoned")
            .insert(*pubkey);
        Ok(Signature::new_unique())
    }

    fn dump_delegated_account(
        &self,
        pubkey: &Pubkey,
        _account: &Account,
        _owner: &Pubkey,
    ) -> AccountDumperResult<Signature> {
        self.delegated_accounts
            .write()
            .expect("RwLock for delegated_accounts is poisoned")
            .insert(*pubkey);
        Ok(Signature::new_unique())
    }

    fn dump_program_accounts(
        &self,
        program_id_pubkey: &Pubkey,
        _program_id_account: &Account,
        program_data_pubkey: &Pubkey,
        _program_data_account: &Account,
        program_idl: Option<(Pubkey, Account)>,
    ) -> AccountDumperResult<Signature> {
        self.program_ids
            .write()
            .expect("RwLock for program_ids is poisoned")
            .insert(*program_id_pubkey);
        self.program_datas
            .write()
            .unwrap()
            .insert(*program_data_pubkey);
        if let Some(program_idl) = program_idl {
            self.program_idls
                .write()
                .expect("RwLock for program_idls is poisoned")
                .insert(program_idl.0);
        }
        Ok(Signature::new_unique())
    }

    fn dump_program_account_with_old_bpf(
        &self,
        program_pubkey: &Pubkey,
        _program_account: &Account,
    ) -> AccountDumperResult<Signature> {
        let programdata_address = get_program_data_address(program_pubkey);

        self.program_ids
            .write()
            .expect("RwLock for program_ids is poisoned")
            .insert(*program_pubkey);
        self.program_datas
            .write()
            .expect("RwLock for program_datas is poisoned")
            .insert(programdata_address);
        Ok(Signature::new_unique())
    }
}

impl AccountDumperStub {
    pub fn was_dumped_as_feepayer_account(&self, pubkey: &Pubkey) -> bool {
        self.feepayer_accounts.read().unwrap().contains(pubkey)
    }
    pub fn was_dumped_as_undelegated_account(&self, pubkey: &Pubkey) -> bool {
        self.undelegated_accounts.read().unwrap().contains(pubkey)
    }
    pub fn was_dumped_as_delegated_account(&self, pubkey: &Pubkey) -> bool {
        self.delegated_accounts.read().unwrap().contains(pubkey)
    }

    pub fn was_dumped_as_program_id(&self, pubkey: &Pubkey) -> bool {
        self.program_ids.read().unwrap().contains(pubkey)
    }
    pub fn was_dumped_as_program_data(&self, pubkey: &Pubkey) -> bool {
        self.program_datas.read().unwrap().contains(pubkey)
    }
    pub fn was_dumped_as_program_idl(&self, pubkey: &Pubkey) -> bool {
        self.program_idls.read().unwrap().contains(pubkey)
    }

    pub fn was_untouched(&self, pubkey: &Pubkey) -> bool {
        !self.was_dumped_as_feepayer_account(pubkey)
            && !self.was_dumped_as_undelegated_account(pubkey)
            && !self.was_dumped_as_delegated_account(pubkey)
            && !self.was_dumped_as_program_id(pubkey)
            && !self.was_dumped_as_program_data(pubkey)
            && !self.was_dumped_as_program_idl(pubkey)
    }

    pub fn clear_history(&self) {
        self.feepayer_accounts.write().unwrap().clear();
        self.undelegated_accounts.write().unwrap().clear();
        self.delegated_accounts.write().unwrap().clear();
        self.program_ids.write().unwrap().clear();
        self.program_datas.write().unwrap().clear();
        self.program_idls.write().unwrap().clear();
    }
}
