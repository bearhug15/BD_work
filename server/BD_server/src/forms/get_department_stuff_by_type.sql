SELECT firstname,secondname,familyname,age
FROM worker
	JOIN department
	ON worker.department_id = department.department_id
WHERE department.name = _departmentname_ AND worker.worker_type = _worker_type_;