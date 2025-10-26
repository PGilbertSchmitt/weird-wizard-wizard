# Understanding Preloading and Eager Loading

Part of my investigation in this project is attempting to understand how eager loading
and preloading work in ORMs, so that I may avoid using them when necessary. One of the
largest mysteries to me has been how ORMs handle associations, especially those that
are nested or combined. I've been writing software for over a decade now, 8 years of
which while employed professionally, yet this kind of behavior is something I've never really thought about until now. What I've uncovered (at least how a major, feature-rich
ORM like ActiveRecord might work) is that there are 2 basic ways to load associations:
**preloading** and **eager loading**.

## Preloading

This is the default (preferred) method that a lot of associated records are pulled. If
I am querying for a `path` and its respective `levels`, I can load the one-to-many
association by separating the request into separate queries for each table without
joining. The idea behind not using `JOIN`s for the associations, which can produce a
lot of extra rows in the results (as seen in the [eager loading](#eager-loading)
section).

```SQL
-- Where $1 is the path ID I'm searching for
SELECT p.*
FROM paths p
WHERE p.id = $1;

SELECT l.*
FROM levels l
WHERE l.path = $1;
```

<!-- But query results here -->

Do note that this example

## Eager Loading

Eager loading differs from preloading because it uses `JOIN`s to collect all associated
records into the same query, then extract them afterwards. This is necessary when there
are conditions that need to be placed on a record that involve values from associated
records. However, this can create much more work for the DBMS because it creates
duplicate data when single records from one table are associated with many records from
another table. Even worse, when multiple associations are used in parallel, you end up
with a cross product. For example, if table A is associated one-to-many to both table B
and table C, then when you query for a record in A with 3 associated B records and 4
associated C records, then the result of eager loading both associations is a total of 12
output records, all for the same A. More associations increases the issues exponentially.

For example, if I am pulling a record from the `ancestries` table, and I also want to
include several many-to-many associations (`speed_traits`, `languages`, `sense`, and
`immunities`), I would not be able to fully use a preloading strategy because the juntion
tables need data from the outer associations. Therefore, I would instead construct an
eager load query like so:

```SQL
SELECT
a.id,
a.name as ancestry,
st.name as speed_traits,
st.unit as speed_unit,
ast.amount as speed_amount,
s.name as senses,
s.unit as sense_unit,
a_s.amount as sense_amount,
i.name as immunities,
l.name as languages
FROM ancestries a
LEFT OUTER JOIN ancestry_speed_traits ast ON a.id = ast.ancestry_id
LEFT OUTER JOIN speed_traits st ON st.id = ast.speed_trait_id
LEFT OUTER JOIN ancestry_senses a_s ON a.id = a_s.ancestry_id
LEFT OUTER JOIN senses s ON s.id = a_s.sense_id
LEFT OUTER JOIN ancestry_immunities ai ON a.id = ai.ancestry_id
LEFT OUTER JOIN immunities i ON i.id = ai.immunity_id
LEFT OUTER JOIN ancestry_languages al ON a.id = al.ancestry_id
LEFT OUTER JOIN languages l ON l.id = al.language_id
WHERE a.name = 'Tatterdemalion';
```

The `Tatterdemalion` ancestry has 4 speed traits and 9 immunities, which compounds to
a sum total of 36 rows just for this one record:

| id  | ancestry       | speed_traits | speed_unit | speed_amount | senses      | sense_unit | sense_amount | immunities  | languages |
| --- | -------------- | ------------ | ---------- | ------------ | ----------- | ---------- | ------------ | ----------- | --------- |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | asleep      |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | blinded     |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | deafened    |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | poisoned    |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | slowed      |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | exposure    |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | deprivation |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | infection   |           |
| 81  | Tatterdemalion | Silent       |            |              | True Vision |            |              | suffocation |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | asleep      |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | blinded     |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | deafened    |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | poisoned    |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | slowed      |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | exposure    |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | deprivation |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | infection   |           |
| 81  | Tatterdemalion | Slippery     |            |              | True Vision |            |              | suffocation |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | asleep      |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | blinded     |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | deafened    |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | poisoned    |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | slowed      |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | exposure    |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | deprivation |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | infection   |           |
| 81  | Tatterdemalion | Squeeze      | inches     | 1            | True Vision |            |              | suffocation |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | asleep      |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | blinded     |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | deafened    |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | poisoned    |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | slowed      |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | exposure    |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | deprivation |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | infection   |           |
| 81  | Tatterdemalion | Strider      |            |              | True Vision |            |              | suffocation |           |

# Combined approach

If we take another look at the `ancestry` query in the eager load example, it can be
shown that the query does not use any conditions that involve the `ancestry` record
itself, only the associations with their junctions. Therefore, both methods can be
combined to reduce the work of the DBMS and the resulting collection: Use preloading
to pull the ancestries and the broad associations, but use `JOIN`s within the
association queries to reduce the work done:

```SQL
-- Where $1 is the ancestry ID I'm searching for
SELECT a.*
FROM ancestries a
WHERE a.id = $1;

-- Query for just the immunities
SELECT l.*
FROM immunities l
JOIN ancestry_immunities ai ON l.id = ai.immunity_id
WHERE ai.ancestry_id = $1;

-- Query for just the languages
SELECT l.*
FROM languages l
JOIN ancestry_languages al ON l.id = al.language_id
WHERE al.ancestry_id = $1;

-- Query for just the speed_traits
SELECT st.*, ast.amount
FROM speed_traits st
JOIN ancestry_speed_traits ast ON st.id = ast.speed_trait_id
WHERE ast.ancestry_id = $1;

-- Query for just the senses
SELECT s.*, a_s.amount
FROM senses s
JOIN ancestry_senses a_s ON s.id = a_s.sense_id
WHERE a_s.ancestry_id = $1;
```

There are 4 queries, but the DBMS only needs to collect 1 ancestry record, 9 speed
trait records, 4 immunity records, 1 sense record, and no language records:

### Ancestry

| id  | name           | descriptor | size | speed | add_health | add_nat_def |
| --- | -------------- | ---------- | ---- | ----- | ---------- | ----------- |
| 81  | Tatterdemalion | Spirit     | sm   | 5     | 0          | 0           |

### Immunities

| id  | name        |
| --- | ----------- |
| 21  | asleep      |
| 22  | deprivation |
| 23  | infection   |
| 25  | poisoned    |
| 26  | exposure    |
| 27  | suffocation |
| 28  | blinded     |
| 29  | deafened    |
| 30  | slowed      |

### Speed traits

| id  | name     | description                                                  | unit   | amount |
| --- | -------- | ------------------------------------------------------------ | ------ | ------ |
| 30  | Slippery | The creature’s moves do not enable other creatures to make f |        |        |
|     |          | ree attacks against it.                                      |        |        |
| 31  | Strider  | The creature reduces by 1 the number of yards of movement it |        |        |
|     |          | expends to move each yard across challenging terrain.        |        |        |
| 32  | Silent   | The creature can sneak 1 yard for each yard of movement it e |        |        |
|     |          | xpends and rolls to sneak with 1 boon.                       |        |        |
| 38  | Squeeze  | The creature can squeeze through openings of the indicated s | inches | 1      |
|     |          | ize.                                                         |        |        |

### Sense

| id  | name        | description                                                  | unit | amount |
| --- | ----------- | ------------------------------------------------------------ | ---- | ------ |
| 17  | True Vision | The creature needs no light to see and treats everything wit |      |        |
|     |             | hin its line of sight as being illuminated. It perceives out |      |        |
|     |             | lines around invisible creatures and objects in its line of  |      |        |
|     |             | sight. It also sees through mundane and magical disguises, p |      |        |
|     |             | erceives transformed creatures in their normal forms, and re |      |        |
|     |             | cognizes visual illusions for what they are.                 |      |        |

This is a lot easier to work with, and to collect into a single structure using code.
