SELECT working_company.company_name, pc_bind.contract_id, pc_bind.project_id, groups_bind.cost+project.cost
FROM working_company
	JOIN groups_bind
	ON working_company.group_id = groups_bind.group_id
	JOIN pc_bind
	ON pc_bind.group_id = working_company.group_id
	JOIN project
	ON pc_bind.project_id = project.project_id;