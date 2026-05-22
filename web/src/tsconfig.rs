```rust
{
  "compilerOptions": {
    "outDir": "./dist",
    "sourceMap": true,
    "target": "ESNext",
    "module": "esnext",
    "strict": true,
    "jsx": "react-jsx",
    "moduleResolution": "node",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "paths": {
      "@lichtblick/suite-base/*": ["../../packages/suite-base/src/*"]
    }
  },
  "include": [
    "./**/*"
  ]
}
```