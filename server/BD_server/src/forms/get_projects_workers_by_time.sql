SELECT worker.worker_id, firstname, secondname, familyname, age
	FROM
	(SELECT contract_id, project_id, group_id
	FROM pc_bind
	WHERE (project_start BETWEEN _timestart_ AND _timeend_) OR  (project_end BETWEEN _timestart_ AND _timeend_)
	) AS in_time
	JOIN groups
	ON groups.group_id = in_time.group_id
	JOIN workers
	ON groups.worker_id = worker.worker_id