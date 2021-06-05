SELECT *
FROM contract INNER JOIN pc_bind
ON contract.contract_id = pc_bind.contract_id;