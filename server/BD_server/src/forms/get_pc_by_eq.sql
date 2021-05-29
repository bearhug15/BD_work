SELECT pc_bind.contract_id, pc_bind.project_id
FROM (SELECT eq_list_id
		FROM eq_group
		WHERE eq_id = _eq_id_) AS lists
		JOIN pc_bind
		ON lists.eq_list_id = pc_bind.eq_list_id;
	