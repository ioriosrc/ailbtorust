```json
{
  "name": "@lichtblick/comlink-transfer-handlers",
  "license": "MPL-2.0",
  "private": true,
  "repository": {
    "type": "git",
    "url": "https://github.com/lichtblick-suite/lichtblick.git"
  },
  "author": {
    "name": "Lichtblick",
    "email": "lichtblick@bmwgroup.com"
  },
  "homepage": "https://github.com/lichtblick-suite",
  "main": "./src/index.rs",
  "files": [
    "dist",
    "src"
  ],
  "scripts": {
    "prepack": "cargo tsc"
  },
  "devDependencies": {
    "@lichtblick/comlink": "1.0.3",
    "@lichtblick/tsconfig": "1.0.2",
    "typescript": "5.9.3"
  }
}
```