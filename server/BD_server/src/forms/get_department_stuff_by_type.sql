SELECT firstname,secondname,familyname,age,worker_type
FROM worker
	JOIN department
	ON worker.department_name = department.department_name
WHERE department.name = _departmentname_ AND worker.worker_type = _worker_type_;