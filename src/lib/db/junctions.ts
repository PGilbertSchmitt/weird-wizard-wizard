import { dbExecute, Tables } from './client';

export const createAncestryLanguage = async (
  ancestryId: number,
  languageId: number,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.ANCESTRY_LANGUAGES} (
  ancestry_id, language_id
) VALUES ($1, $2)`,
    [ancestryId, languageId],
  );
};

export const createLevelLanguage = async (
  levelId: number,
  languageId: number,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.LEVEL_LANGUAGES} (
  level_id, language_id
) VALUES ($1, $2)`,
    [levelId, languageId],
  );
};

export const createAncestrySpeedTrait = async (
  ancestryId: number,
  speedTraitId: number,
  amount: string | null,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.ANCESTRY_SPEED_TRAITS} (
  ancestry_id, speed_trait_id, amount
) VALUES ($1, $2, $3)`,
    [ancestryId, speedTraitId, amount],
  );
};

export const createLevelSpeedTrait = async (
  levelId: number,
  speedTraitId: number,
  amount: string | null,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.LEVEL_SPEED_TRAITS} (
  level_id, speed_trait_id, amount
) VALUES ($1, $2, $3)`,
    [levelId, speedTraitId, amount],
  );
};

export const createAncestrySense = async (
  ancestryId: number,
  senseId: number,
  amount: string | null,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.ANCESTRY_SENSES} (
  ancestry_id, sense_id, amount
) VALUES ($1, $2, $3)`,
    [ancestryId, senseId, amount],
  );
};

export const createAncestryImmunity = async (
  ancestryId: number,
  immunityId: number,
) => {
  await dbExecute(
    `
INSERT INTO ${Tables.ANCESTRY_IMMUNITIES} (
  ancestry_id, immunity_id
) VALUES ($1, $2)`,
    [ancestryId, immunityId],
  );
};
