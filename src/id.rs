use hedera::{AccountId, ContractId, FileId};

def_to_str!(hedera_account_id_to_str: AccountId);
def_from_str!(hedera_account_id_from_str: AccountId);

def_to_str!(hedera_contract_id_to_str: ContractId);
def_from_str!(hedera_contract_id_from_str: ContractId);

def_to_str!(hedera_file_id_to_str: FileId);
def_from_str!(hedera_file_id_from_str: FileId);
