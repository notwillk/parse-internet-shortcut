const test = require('node:test');
const assert = require('node:assert/strict');
const { access, readdir, readFile } = require('node:fs/promises');
const path = require('node:path');

const { parseInternetShortcut } = require('../index');

const fixturesDir = path.join(__dirname, '..', 'fixtures');

test('all .url fixtures parse to their matching .json snapshots', async () => {
  const fixtureNames = (await readdir(fixturesDir))
    .filter((fileName) => fileName.endsWith('.url'))
    .sort();

  assert.ok(fixtureNames.length > 0, 'Expected at least one .url fixture');

  await Promise.all(
    fixtureNames.map(async (fixtureName) => {
      const baseName = fixtureName.slice(0, -4);
      const expectedPath = path.join(fixturesDir, `${baseName}.json`);

      await assert.doesNotReject(
        access(expectedPath),
        `Missing JSON snapshot for fixture ${fixtureName}`,
      );

      const [rawShortcut, rawExpected] = await Promise.all([
        readFile(path.join(fixturesDir, fixtureName), 'utf8'),
        readFile(expectedPath, 'utf8'),
      ]);

      assert.deepStrictEqual(
        parseInternetShortcut(rawShortcut),
        JSON.parse(rawExpected),
        `Fixture ${fixtureName} did not match ${baseName}.json`,
      );
    }),
  );
});
