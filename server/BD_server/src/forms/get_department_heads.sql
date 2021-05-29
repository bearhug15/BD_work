SELECT department.name, firstname,secondname,familyname
FROM department
	JOIN department_head
	ON department.department_id = department_head.department_id
	JOIN worker
	ON department_head.worker_id = worker.worker_id;
