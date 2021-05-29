SELECT DISTINCT contract_id
FROM pc_bind
WHERE (_timestart_ BETWEEN project_start AND project_end) OR (_timeend_ BETWEEN project_start AND project_end);