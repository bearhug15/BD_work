SELECT sub_res.contract_id
	FROM 
	(SELECT contract_id, group_id
		FROM pc_bind
		WHERE project_start<= _timeend_ OR project_end >= _timestart_) AS sub_res
		JOIN groups
		ON sub_res.group_id = groups.group_id
		JOIN worker
		ON worker.worker_id = groups.worker_id
WHERE worker.worker_id = _worker_id_;