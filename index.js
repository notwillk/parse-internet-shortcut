const { readFile } = require('node:fs/promises');

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
        result[currentSection] = {};
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
      result[currentSection][key] = value;
      continue;
    }

    result[key] = value;
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
