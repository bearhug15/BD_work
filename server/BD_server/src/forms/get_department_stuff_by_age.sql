SELECT firstname,secondname,familyname,age
FROM worker
	JOIN department
	ON worker.department_id = department.department_id
WHERE department.name = _departmentname_ AND worker.age >= _agestart_ AND worker.age<= _ageend_;