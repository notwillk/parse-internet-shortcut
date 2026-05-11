# parse-internet-shortcut

Parse Windows `.url` internet shortcut files into plain JavaScript objects.

## Usage

```js
const { parseInternetShortcut } = require('parse-internet-shortcut');

const parsed = parseInternetShortcut(`
[InternetShortcut]
URL=https://example.com/
IconIndex=0
`);
```

## Tests

Fixture-based integration coverage lives in `/fixtures`. Each `.url` file has a matching `.json` snapshot, and `npm test` validates every pair automatically.
