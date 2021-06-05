SELECT department.name, worker.worker_id, firstname,secondname,familyname
FROM department
	JOIN department_head
	ON department.department_name = department_head.department_name
	JOIN worker
	ON department_head.worker_id = worker.worker_id;
