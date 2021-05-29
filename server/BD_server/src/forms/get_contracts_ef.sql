SELECT contract.contract_id, contract.cost/(EXTRACT(DAY from contract.contract_end - contract.contract_start))
FROM contract;
