import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: 'http://das.tail25b5a0.ts.net:4000/openapi.json',
  output: {
    path: 'src/client',
  },
  plugins: [
    '@hey-api/typescript',
    {
      name: '@hey-api/client-fetch',
      runtimeConfigPath: './src/hey-api.ts',
    },
    '@hey-api/sdk',
  ],
});
