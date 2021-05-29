SELECT DISTINCT worker.worker_id, worker.firstname, worker.secondname, worker.familyname
FROM (SELECT group_id 
		FROM pc_bind
		WHERE contract_id = _contract_id_) AS gr
		JOIN groups 
		ON groups.group_id = gr.group_id
		JOIN worker
		ON groups.worker_id = worker.worker_id;
	
		