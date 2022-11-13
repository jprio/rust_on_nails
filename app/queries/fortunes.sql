--! fortunes
/*
This will generate a function called fortunes which will run the SQL query. Note cornucopia checks the query at code generation time against Postgres.
*/
SELECT 
    id, message
FROM 
    Fortune;