# MOD DSL Notes

(note: These notes were written before and during the implementation of the Mod DSL parser, and may be out of date. The parser itself is a better source of truth for the intended behavior of the DSL language.)

A paper pad for toying with the MOD DSL, which is a simple _**D**omain **S**pecific **L**anguage_ that is used by certain abilities to augment the player character's STTs (stat/trait/talent). These are provided by talents and spells via a `mod` column in the database, and can do things like increase ability scores, provide access to languages and immunities, alter previously acquired talents, etc. Theres a large element of complexity involved, as a lot of these require choices to be made by the player, either when selecting the talent (_"you must now pick a new spell from a specific tradition"_), or when casting an ability (_"on casting this spell, pick a benefit from this table"_). Some of these choices are permanent (at least, permanent in the sense that _"as long as you have this talent on your character, you gain this benefit"_), some last a specific amount of time, and some can be dispelled manually or toggled at-will. Because of this complexity, it was decided to use a DSL to encode this behavior.

There are several base MOD types:

- `NONE` - The null case, useful when I need to explicitly provide a MOD that does nothing (which helps with finding holes during import).
- `GRANT` - provides an STT alteration to the PC. This is a terminal MOD.
- `LOSE` - similar to GRANT, but removes an STT (useful for talents that replace other talents). A LOSE MOD has higher priority than a GRANT. This is a terminal MOD.
- `OVERRIDE` - similar to GRANT, but rather than adding something, it replaces it. Some are dismissable, but not always. This is how transformations are performed. Because it's not additive, OVERRIDEs have higher priority than LOSE. This is a terminal MOD.
- `CHOOSE` - associates the player with a choice that must be resolved. These are tracked in the database. To resolve a choice, it must be paired with another MOD. Usually, this is a GRANT, but some choice options will allow you to make another choice, which means that this is NOT a terminal MOD. A CHOOSE MOD effectively forms a tree of possibilities, and the leaves of this tree must all be terminal mods in order for the choices to be resolved.
- `APPLY` - provides an immediate effect once, such as healing or restoring spell slots. This is a terminal MOD, and is either attached to a cast ability, or is placed on a `CAST` choice (because a `PERM` choice providing a one-off effect doesn't really make any sense).

There is also the WHEN MOD, though this isn't a base MOD. Rather it's the first element of a group of sub-MODs within a single MOD entry (except for APPLY). The WHEN MOD details when the GRANT/LOSE/OVERRIDE/CHOOSE is applied. There are 4 kinds of WHEN:

- `PERM` - When the talent is acquired, not dismissable (but can be reset by removing the talent)
- `CAST` - When the talent or spell is cast, and can be dismissed manually
- `CAST {Time}` - When the talent or spell is cast, and can only be dismissed after the passage of time
- `CAST! {Time}` - When the talent or spell is cast, and can be dismissed both manually and after the passage of time

So if you have `[PERM] GRANT.etc...`, it's a GRANT that is applied as soon as it's associated with the player. A `[CAST] CHOOSE.etc` is a choice that made once the player casts the talent or spell, rather than right away. Since a WHEN mod applies to a collection of MODs for a single talent/spell, that means all the individual sub-MODs in that entry will have the same details. It does not make much sense for a talent to both grant a permanent attribute and a castable attribute. I haven't come across any yet in the official rules. Spells never provide passives, and talents can only be acquired once, so there's no worrying about spell-slot math. If a talent providing both a PERM and a CAST is truly required, then it can be resolved by splitting the given talent into 2. I also haven't run into any situations where a talent or spell's CAST can have multiple effects with differing dismissal properties (eg Fly for 1 hour and Slippery for 1 minute). If I do find that edge case, I can pull some new syntax out of my ass.

## GRANT

The simplest type of MOD entry is a permanent effect called a GRANT, so called because it grants an STT. GRANTs are the end-result of any string of choices; a CHOOSE can expose a selection of both CHOICEs and GRANTs. In a CHOOSE tree, consider the GRANTs to be leaves.

The pattern for a GRANT is `GRANT.WHEN.{Category}.{Value}`. A single GRANT is internally delimited by periods, and a cluster of GRANTs together is delimited by a semicolon. In this list, I attempt to put as many of the known simple categories that follow this pattern:

```
[PERM] GRANT.INT.1 ;
[PERM] GRANT.STR.1 ;
[PERM] GRANT.WILL.1 ;
[PERM] GRANT.AGL.1 ;
[PERM] GRANT.Speed.2 ;
[PERM] GRANT.Health.5 ;
[PERM] GRANT.Defense.2 ;
[PERM] GRANT.NatDef.2 ;
[PERM] GRANT.Language.Sylvan ;
[PERM] GRANT.Tradition.War ;
[PERM] GRANT.Talent.Fighter.Brute ;
[PERM] GRANT.MagicTalent.Illusion.Illusiary Image - Alter Reality ;
[PERM] GRANT.SpeedTrait.Slippery ;
[PERM] GRANT.Sense.Keen Sight ;
[PERM] GRANT.Immunity.exposure ;
[PERM] GRANT.BonusDamage.1 ;
```

Note the `PERM` value for the WHEN mod. That means all these GRANTs would be applied to the player instantly. If instead, they were `CAST`, that would mean that the GRANTs only apply once the talent/spell is cast by the player:

```
[CAST] GRANT.Defense.15 ;
[CAST] GRANT.SpeedTrait.Slippery ;
```

These can be dismissed at any time by the player, though not all cast effects can be dismissed immediately, see the notes [here](#note-about-cast).

These are shown in this document ending with a semicolon, but that is not strictly necessary. A semicolon is only required when separating MODs in a single MOD entry (which can contain several MOD units). Here is a single result which grants both a bonus to INT and access to the Sylvan language:

```
[PERM] GRANT.INT.1 ; GRANT.Language.Sylvan
```

There are additional GRANT categories that use a longer pattern to add an extra sub-category. One kind is the `Spell` category, because it's possible for 2 spells in different traditions to have the same name. There is only 1 official entry where this is true (called 'My Spying Eye', found in both the Skullduggery and Technomancy traditions), and it would be cool to also support unofficial entries which share the same circumstances.

```
[PERM] GRANT.Spell.Technomancy.My Spying Eye ;
[PERM] GRANT.Spell.Cryomancy.Ice Barrier ;
```

This also applies to the `Talent` category, which requires providing the source (either an Ancestry or Path):

```
[PERM] GRANT.Talent.Fighter.Fighting Style: Brute ;
```

The `Tradition` GRANT category is one which necessitates an additional choice (picking a talent from that Tradition), so it's not likely to be found without a separate CHOOSE entry unless the option granting a tradition does so with the express intent that the character does NOT pick a talent from this tradition (which would effectively just say "you can now pick spells from this tradition"). I do not yet know if any official options will require this, so you should expect any normal tradition choice to be accompanied by its equivalent CHOOSE:

```
[PERM] GRANT.Tradition.Chaos ; CHOOSE.SpellTalent.Chaos ;
```

If for some stupid reason the Value contains a period, it can be wrapped in quotes. I don't think there is an official entry with a period, but it's good to be prepared for custom shenanigans:

```
[PERM] GRANT.Talent.'Not a real talent.' ;
```

### Note about `CAST`

Because GRANTs (and other MODs) that include `CAST` are inherently temporary, that means they are dismissable. A spell that increases defense by 5 that can be freely dismissed would simply be:

```
GRANT.CAST.Defense.5 ;
```

However, Some GRANT effects can only be dismissed after a certain amount of time has passed. These have the pattern of `[CAST({Time})]GRANT.{Category}.{Value}`. This example is for a GRANT that provides the access to the Fly speed trait for 1 minute:

```
[CAST(1m)] GRANT.SpeedTrait.Fly
```

The possible time values are:

- `1m` - 1 minute
- `1h` - 1 hour
- `4h` - 4 hours
- `8h` - 8 hours
- `1d` - 1 day
- `rest` - Until rest
  I don't feel like implementing full-on time tracking, so this is as granular as it needs to be. As far as I can see, these were all of the possible lengths of time for an effect to last in the official rules.

If a CAST can be manually dismissed **and** has a time limit, then a `!` can be added to the CAST:

```
GRANT.CAST!(1h).Sense.Dark Vision,Keen Hearing ;
```

## LOSE

A LOSE MOD is just like a GRANT, except it takes away something. Currently, this is only used for talents that are replaced with other talents, so you'll often find it combined with a GRANT for the updated talent:

```
[PERM] LOSE.Talent.Luminous Symbiote ; GRANT.Talent.Durable Luminous Symbiote ;
```

The benefit of using LOSE to replace talents instead of OVERRIDE is that any talents lost in this way will still show up on the sheet, but they will be greyed out and marked as having been overriden.

## OVERRIDE

These are a terminal option similar to GRANTs, except instead of adding something, something is overriden. Sometimes, it's a single stat like defense changing to a new value, but the bigger usage of this MOD is to temporarily replace the sheet with a statblock during a transformation. Some transformations are dismissable, but not all. The dismissable nature of the OVERRIDE is part of the definition. For example, for OVERRIDEs that can't be dismissed, the pattern is simply `[PERM] OVERRIDE.{Category}.{Value}`, since a WHEN of `PERM` means the OVERRIDE is applied to the player immediately, and these are not directly dismissable (though they could still be dismissed if a parent CHOOSE was itself dismissed). This example is for the `Bone Carapace`, which overrides the defense to 18:

```
[PERM] OVERRIDE.Defense.18 ;
```

(NOTE: This spell also has a condition attached to the override, but this has been omitted for this example. See [conditions](#conditions) section for more details)

The Warg can enter and leave its `Wolf Form` as an action, which means it's freely dismissable. This is denoted by using WHEN of `CAST` without a time value:

```
OVERRIDE.StatBlock.Wolf From ;
```

The transformation from the primal spell `Howl of the Beast` cannot be dismissed manually and only goes away after 1 hour has passed, so it requires a WHEN of `CAST` with a time value:

```
OVERRIDE.StatBlock.Howl of the Beast ;
```

An uncommon case for an OVERRIDE is to alter a statblock that the player is expected to have in some form. Many instances of this are more easily performed as a LOSE+GRANT (eg a choice includes a LOSE for an old talent with a statblock, and a GRANT for a talent with an updated statblock). However, the Beastfriend path at level 4 asks the player to make a choice that alters the statblock of a beast companion. There are 5 possible configurations of the companion at level 4, and levels 6 and 9 further update the stats. With the initial statblock at level 3, there would need to be 16 total versions of the talent (one for each possible statblock) depending on level and decisions. Rather than tracking all that with LOSE+GRANT (which would be possible but very tedious), it's a lot easier to define 1 CHOOSE block that results in a single OVERRIDE, then a couple more overrides at levels 6 and 9.

For a simpler contrived example, let's say a path talent is associated with a statblock defined as:

| Cool Guy | Stat       | Value                                                    | BLOCK |
| -------- | ---------- | -------------------------------------------------------- | ----- |
| Cool Guy | ID         | Cool Guy                                                 |       |
| Cool Guy | Attributes | STR 15, AGL 14, INT 12, WILL 17                          |       |
| Cool Guy | Defense    | 15                                                       |       |
| Cool Guy | Health     | 30                                                       |       |
| Cool Guy | Speed      | 5                                                        |       |
| Cool Guy | Senses     | Keen Hearing                                             |       |
| Cool Guy | Punch      | The Cool Guy does a sick punch that can deal 2d6 damage. |       |

At a later level for that path, the statblock is updated to have 35 health and a modified `Punch` ability. A separate statblock is defined as:

| Cooler Guy | Stat   | Value                                                                                    | BLOCK |
| ---------- | ------ | ---------------------------------------------------------------------------------------- | ----- |
| Cooler Guy | Name   | Cooler Guy                                                                               |       |
| Cooler Guy | Health | 35                                                                                       |       |
| Cooler Guy | Punch  | The Cool Guy does a sick punch that can deal 3d6 damage. He rolls to attack with 1 boon. |       |

The talent that modifies the statblock can have a mod to apply the new statblock onto the original:

```
[PERM] OVERRIDE.Merge.'Cool Guy'.'Cooler Guy'
```

This causes any stats on the `Cooler Guy` statblock to override the `Cool Guy` statblock, which in this case is the `Health`, `Punch`, and `Name` attributes. The ID of a statblock does not change after this kind of override. All statblocks rely on the ID for the label, so in order to override this label, an explicit `Name` attribute can be provided as shown.

If an override statblock needs to remove any traits, it can be done by providing that trait with a blank description. Here's a different version of the Cooler Guy that loses it's punch but gains a kick:

| Cooler Guy | Stat   | Value                                                                                      | BLOCK |
| ---------- | ------ | ------------------------------------------------------------------------------------------ | ----- |
| Cooler Guy | Name   | Cooler Guy                                                                                 |       |
| Cooler Guy | Health | 35                                                                                         |       |
| Cooler Guy | Kick   | The Cool Guy does a radical kick that can deal 3d6 damage. He rolls to attack with 1 boon. |       |
| Cooler Guy | Punch  |                                                                                            |       |

Some statblock merge traits are additive rather than replacing the previous version: `Senses`, `Immunities`, `Languages`, or `Speed Traits` add to the previous list by default. For example, let's give the Cool Guy the `Keen Vision` trait:

```
[PERM] OVERRIDE.Merge.'Cool Guy'.'Cool Guy With Good Vision'
```

| Cool Guy With Good Vision | Stat   | Value       | BLOCK |
| ------------------------- | ------ | ----------- | ----- |
| Cool Guy With Good Vision | Senses | Keen Vision |       |

This results in a Cool Guy with both `Keen Hearing` and `Keen Vision`.

This behavior can be overwritten by preceeding the trait name with `!`. If instead the merge statblock was defined like so:

| Cool Guy With Good Vision | Stat    | Value       | BLOCK |
| ------------------------- | ------- | ----------- | ----- |
| Cool Guy With Good Vision | !Senses | Keen Vision |       |

Then it would result in a Cool Guy with `Keen Vision` but without `Keen Hearing`.

`Speed`, `Health`, `Defense`, and other stats that are purely numeric in nature replace by default, but can easily be made additive with `+`. Here's a merge block for `Cool Guy` that increases speed by 1:

| Fast Guy | Stat  | Value | BLOCK |
| -------- | ----- | ----- | ----- |
| Fast Guy | Speed | +1    |       |

The resulting Cool Guy would have a speed of 6.

## CHOOSE

This is the big one. A CHOOSE MOD is what provides talents and spells with the ability to provide the player with agency. A spell that has a CHOOSE will generally apply when the spell is cast, while talents have a healthy mix between on-cast choices and permanent decisions that are only applied the moment the talent is acquired. Similar to OVERRIDEs, some choices are made immediately and permanently, which have the WHEN of `PERM`, while others, like the Jann's `Elemental Affinity` talent, are choices that are made once the talent or spell is CAST. Sometimes, the choice itself is timelocked, and this can mean the `CAST` has a time value, while other choices might defer the dismissal to the options.

A CHOOSE MOD will, once associated with the player, create a pending selection that the player must address until the choice is satisfied. This is managed by the `player_choice` table. The `choice` column will include the choice string, and the `selection` column will hold the corresponding selection made by the player, which can be any other MOD string. A NULL value means no choice was made yet.

For a simple example of a permanent choice made as soon as the talent is selected, consider a talent that lets you permanently pick any tradition:

```
[PERM] CHOOSE.Tradition ;
```

A tradition is simple to direct the player to acquire because the Tradition form is already present. Other simple forms of permanent choices are:

```
[PERM] CHOOSE.NoviceSpell ;
[PERM] CHOOSE.ExpertSpell ;
[PERM] CHOOSE.MasterSpell ;
[PERM] CHOOSE.Language ;
[PERM] CHOOSE.Profession ;
[PERM] CHOOSE.Score ;
```

Choices for spells assume that they only pick from any of the PC's current traditions, but this can be overriden. A choice for a spell from any tradition is:

```
[PERM] CHOOSE.NoviceSpell.ANY ;
```

A choice for a spell from 1 specific tradition is:

```
[PERM] CHOOSE.NoviceSpell.Chronomancy ;
```

And a choice for a spell from a subset of traditions is:

```
[PERM] CHOOSE.NoviceSpell.Aeromancy,Geomancy,Hydromancy,Pyromancy ;
```

If multiple selections are to be made at the same time, you can use parens with a number after the category:

```
[PERM] CHOOSE.NoviceSpell(2) ;
CHOOSE.CAST.SELECT(2)=Elemental Affinities ;
```

Most choices require the player to choose from a curated list. The pattern is `[PERM] CHOOSE.SELECT={ChoiceSelection}`. The `ChoiceSelection` is the name of an entry in the `choice_selections` table, which is similar to the `option_blocks` table with an extra `mod` column (which is the MOD that is applied to the player once a selection has been made). consider the Clockwork's `Clockwork Upgrade` talent. This requires the player to make an immediate choice of selecting a benefit that remains with their character:

```
[PERM] CHOOSE.SELECT=Clockwork Upgrade ;
```

This would direct the system to present the user with a form to pick from one of several options, source from the `choice_selections` table. Let's say the source table had entries like this:

| `choice_name`     | `selection_text`                                                                                                  | `mod`                                                      |
| ----------------- | ----------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| Clockwork Upgrade | You roll to attack with 1 boon.                                                                                   | `[PERM] GRANT.Talent.Clockwork Upgrade Boon`               |
| Clockwork Upgrade | You discover one tradition.                                                                                       | `[PERM] CHOOSE.Tradition`                                  |
| Clockwork Upgrade | Gain a profession of your choice, increase your Intellect by 1, and add a language to the list of those you know. | `[PERM] CHOOSE.Profession ; GRANT.Int.1 ; CHOOSE.Language` |

The player would be shown the list of option texts and promped to select 1. The selection will store the corresponding `mod` value in the `player_choices` table, then those choices would apply themselves to the player. Should any of them also be choices, that will instantiate new choice entries in the `player_choices` table, and the player will need to make yet more selections.

A specialty type of choice is `Slot`. Some abilities can temporarily modify the number of slots for a known spell. Here's a CHOOSE mod that lasts until the next rest and increases a selected spell's slot count by 2:

```
[CAST REST] CHOOSE.Slot.+2
```

Here's the same mod that doubles a selected spell's slot:

```
[CAST REST] CHOOSE.Slot.*2
```

The only options here are to add or to multiply.

There are of course castable (dismissable) choices too. The syntax for these maintains the nature of `CAST`. This is a CHOOSE which lets you pick any novice spell, and the selection will last for 8 hours.

```
[CAST! 8h] CHOOSE.NoviceSpell
```

### Distinct Selections

Choices fall into 3 categories of distinct selection:

- Options for spells and traditions can be picked multiple times (Pick 2 spells -> you can take the same spell twice)
- Options for ability scores are uniq within a single selection (Pick 2 scores -> You must take different scores this time, but if taking another score selection later, the same scores can be picked again)
- Options for languages, professions, or from a table are universally distinct (If an option from a table is picked, it cannot be picked again).

For table selections, the default behavior can be overriden; if options in a table selection can be picked a second time, then instead of `SELECT=`, you can use `SELECT_AGAIN=`:

```
PERM.CHOOSE.SELECT_AGAIN=Some table ;
```

(I'm not actually sure this is needed for the official talents, but it doesn't seem too difficult to support for homebrew)

## APPLY

The APPLY MOD performs a single action which immediately affects the PC. Any choice or cast that exhibits an APPLY. The common form of this is healing:

```
APPLY.Heal.5 ;
APPLY.Heal.3d6 ;
APPLY.Heal.Calc(LEVEL*2) ;
APPLY.Heal.Calc(DMG / 2) ;
APPLY.Heal.Ask("Prompt text") ;
```

The Heal clause generally encloses an simple expression, using the same data sources that the conditionals have access to:

- Any numbers
- `LEVEL` = Current level
- `DMG` = The current damage amount
  But there's a special case where it asks for a value from the player, which requires a prompt:

```
APPLY.Heal.Ask("...") ;
```

Heal heals damage, but the health stat can also be restored by using `HEALTH`:

```
APPLY.Health.10 ;
APPLY.Health.2d6 ;
APPLY.Health.Calc(HEALTH*2) ;
APPLY.Health.Ask("...") ;
```

It can also restore spell slots by supplying `Slot` with the category, which can be comma separated:

```
APPLY.Slot.ANY ;
APPLY.Slot.ANY(2) ;
APPLY.Slot.NoviceSpell ;
APPLY.Slot.NoviceSpell(2) ;
APPLY.Slot.ExpertSpell ;
APPLY.Slot.MasterSpell ;
APPLY.Slot.ExpertSpell(2)
APPLY.Slot.Technomancy ;
APPLY.Slot.Technomancy(2) ;
APPLY.Slot.Technomancy.My Spying Eye ;
APPLY.Slot.Technomancy(2).My Spying Eye ;
```

## Conditions

Some grants or overrides can only apply when conditions are met on the character, and likewise, some choices or choice options depend on conditions to be allowed to be selected. To support these as arbitrary MODs, we can introduce conditional syntax.

The syntax as additive to the base MOD, and if the condition holds, the MOD applies. The syntax has the pattern `{MOD}@{Condition}.{Category}.{Value}`. The Condition/Value section matches the syntax of the GRANTs.

(#TODO, get list of all possible categories that can be checked for a PC)

In many cases, you may need to have multiple MODs together where one MOD is applied when a specific condition is met, and another MOD is applied when the inverse of that condition is met. For example, the Hydromancy talent `Sea Heart` gives you the `Slippery` trait, but if you already have it, then it increases your speed by 2 instead:

```
GRANT.PERM.SpeedTrait.Slippery@HASNOT.SpeedTrait.Slippery ; GRANT.PERM.Speed.2@HAS.SpeedTrait.Slippery ;
```

This grants the slippery trait if the PC does not have it already. This can be shortened slightly using a shorthand: if you're granting something only if the PC doesn't already have it, you can just write the `@HASNOT` condition without a Category or Value:

```
GRANT.PERM.SpeedTrait.Slippery@HASNOT ; GRANT.PERM.Speed.2@HAS.SpeedTrait.Slippery ;
```

A condition can check more than whether the the PC has or doesn't have a trait. There are situations where the level or the value of a stat can alter selections. The `Hunter's Senses` trait grants one of 3 sense traits. At level 3, it grants another one of those traits (distinct from the first). At level 7, it grants the last unchosen trait. In order to make this work, the MOD for the talent uses 3 separate CHOICEs, all pointing to the same list but with different conditions against the level:

```
CHOOSE.PERM.Hunter's Senses ; CHOOSE.PERM.Hunter's Senses.DISTINCT@IF(LEVEL >= 3) ; CHOOSE.PERM.Hunter's Senses.DISTINCT@IF(LEVEL >= 7) ;
```
