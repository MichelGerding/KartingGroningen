-- get the dll ordered by
SELECT ddl
from (SELECT *
      FROM (
               -- get the dll for the tables
               SELECT 0 AS prio,
                      ddl
               FROM (
                        -- generate the dll
                        SELECT 0              AS prio,
                               -- convert the table info into create statement
                               'CREATE TABLE public.'
                                   || tabledefinition.table_name
                                   || E' ( \n'
                                   || array_to_string(
                                       array_agg(
                                                                   E'\t'
                                                                   || tabledefinition.column_name
                                                               || ' '
                                                           || tabledefinition.type
                                                       || ' '
                                                   || tabledefinition.not_null
                                           ), E',\n')
                                   || E'\n);' AS ddl,

                               tabledefinition.table_name
                        FROM (SELECT table_name,
                                     column_name,
                                     data_type   as type,
                                     CASE
                                            WHEN is_nullable = 'NO' THEN 'NOT NULL'::text
                                            ELSE 'NULL'::text
                                            END                              AS not_null

                              FROM information_schema.columns
                              ORDER BY ordinal_position) tabledefinition
                             -- get the information of the table
--                                  SELECT c.relname                            AS TABLE_NAME,
--                                         a.attname                            AS COLUMN_NAME,
--                                         format_type(a.atttypid, a.atttypmod) AS TYPE,
--                                         CASE
--                                             WHEN a.attnotnull THEN 'NOT NULL'::text
--                                             ELSE 'NULL'::text
--                                             END                              AS not_null
--                                  FROM pg_class c,
--                                       pg_attribute a,
--                                       pg_type t
--                                  WHERE a.attnum > 0
--                                    AND a.attrelid = c.oid
--                                    AND a.atttypid = t.oid
--                                  ORDER BY a.attnum) tabledefinition
                        GROUP BY tabledefinition.table_name) AS gettabledll
               WHERE TABLE_NAME IN (
                   -- get the tablenames that are public
                   SELECT t.tablename
                   FROM pg_tables t
                   WHERE t.schemaname = 'public'::name)) AS tables
           -- union with the dll of the constraints and keys
      UNION
      SELECT 2                            AS prio,
             'ALTER TABLE ' || table_from || ' ADD ' ||
             CASE
                 WHEN is_constraint
                     THEN 'CONSTRAINT ' || constraint_name || '_ '
                 ELSE ''
                 END || partialDDl || ';' AS ddl
      FROM (
               -- get information about the constraints
               SELECT conname                   AS constraint_name,
                      conrelid::regclass        AS table_from,
                      pg_get_constraintdef(oid) as partialDDl,
                      CASE
                          WHEN pg_get_constraintdef(oid) LIKE 'UNIQUE %'
                              THEN true
                          ELSE false
                          END                   AS is_constraint
               FROM pg_constraint
               WHERE contype IN ('f', 'p ', 'u')
                 AND connamespace = 'public'::regnamespace) AS keydef
      UNION
      -- get the indexes which provide the dll
      SELECT 1                          AS prio,
             pg_indexes.indexdef || ';' AS ddl
      FROM pg_indexes
      WHERE pg_indexes.schemaname = 'public'::name
      UNION
      SELECT 3                                                                             AS prio,
             'CREATE VIEW ' || vs.view || ' AS' || pg_get_viewdef(vs.view::regclass, true) AS dd
      FROM (SELECT table_schema AS schema,
                   table_name   AS view
            FROM information_schema.views
            WHERE table_schema = 'public'
            ORDER BY schema, view) AS vs
      ORDER BY prio) AS generateSchema;


