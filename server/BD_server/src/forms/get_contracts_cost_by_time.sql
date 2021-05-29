SELECT SUM(cost)
FROM contract
WHERE contract_start>= _timestart_ AND contract_end<= _timeend_ ;