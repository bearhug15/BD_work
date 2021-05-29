SELECT equipment.eq_id, equipment.name, sub_res.project_start, sub_res.project_end
FROM (SELECT eq_list_id, project_start,project_name 
		FROM pc_bind
		WHERE project_id = _id_) AS sub_res
	JOIN eq_group
	ON eq_group.eq_list_id = sub_res.eq_list_id
	JOIN equipment
	ON equipment.eq_id = eq_group.eq_id;