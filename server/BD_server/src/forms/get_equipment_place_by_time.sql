SELECT equipment.eq_id, equipment.name, sub_res.contract_id, sub_res.project_id
FROM 
	(SELECT contract_id, project_id, eq_list_id
	FROM pc_bind 
	WHERE _time_ BETWEEN project_start AND project_end) AS sub_res
	JOIN eq_group
	ON eq_group.eq_list_id = sub_res.eq_list_id
	JOIN equipment
	ON eq_group.eq_id = equipment.eq_id;