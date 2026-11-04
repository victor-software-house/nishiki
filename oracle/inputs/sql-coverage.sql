  -- Written for nishiki: what the sql grammar's other inputs leave out.
  # A hash line.
CREATE TABLE events (
  id int,
  ratio float(10),
  starts time(3) with time zone,
  ends timestamptz(6) without time zone,
  label [name]]part],
  note [outer[inner]
);
SELECT 1; DROP TABLE events CASCADE;
GRANT SELECT ON events TO reader;
REVOKE SELECT ON events FROM reader;
SELECT cume_dist() OVER (ORDER BY id), bit_count(id), collationproperty('Latin1_General_CI_AS', 'CodePage');
SELECT hashbytes('SHA2_256', 'x'), cursor_status('global', 'c'), @@error(), json_value('{}', '$.a');
SELECT iif(1 > 0, 'yes', 'no'), user_name(), newid(), textptr(note), vector_distance('cosine', a, b);
SELECT `plain`, `a\`b`, "a#{b}c", %{text}, %r{ab#{c}d} FROM events;
SELECT 4 / 2 / 1;
