SELECT worker_id, firstname,secondname,familyname,age
FROM worker
WHERE worker.age >= _agestart_ AND worker.age <= _ageend_
ORDER BY worker_id;