SELECT contract_id
FROM contract
WHERE (_timestart_ BETWEEN contract_start AND contract_end) OR (_timeend_ BETWEEN contract_start AND contract_end);