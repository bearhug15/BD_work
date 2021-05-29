SELECT SUM(project.cost)
FROM pc_bind
	JOIN project
	ON project.project_id = pc_bind.project_id
WHERE project_start>= _timestart_ AND project_end<= _timeend_;