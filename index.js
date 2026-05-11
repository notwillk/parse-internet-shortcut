const { readFile } = require('node:fs/promises');

function defineSafeProperty(target, key, value) {
  Object.defineProperty(target, key, {
    value,
    enumerable: true,
    writable: true,
    configurable: true,
  });
}

function parseInternetShortcut(input) {
  if (typeof input !== 'string') {
    throw new TypeError('Expected internet shortcut contents to be a string');
  }

  const result = {};
  let currentSection = null;

  for (const rawLine of input.replace(/^\uFEFF/, '').split(/\r?\n/)) {
    const line = rawLine.trim();

    if (!line || line.startsWith(';') || line.startsWith('#')) {
      continue;
    }

    const sectionMatch = line.match(/^\[(.+)]$/);
    if (sectionMatch) {
      currentSection = sectionMatch[1];
      if (!Object.hasOwn(result, currentSection)) {
        defineSafeProperty(result, currentSection, {});
      }
      continue;
    }

    const separatorIndex = line.indexOf('=');
    if (separatorIndex === -1) {
      continue;
    }

    const key = line.slice(0, separatorIndex).trim();
    const value = line.slice(separatorIndex + 1).trim();

    if (currentSection) {
      defineSafeProperty(result[currentSection], key, value);
      continue;
    }

    defineSafeProperty(result, key, value);
  }

  return result;
}

async function parseInternetShortcutFile(filePath, encoding = 'utf8') {
  const contents = await readFile(filePath, encoding);
  return parseInternetShortcut(contents);
}

module.exports = {
  parseInternetShortcut,
  parseInternetShortcutFile,
};
