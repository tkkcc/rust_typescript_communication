import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  client: '@hey-api/client-fetch',
  input: '../server/openapi.yaml',
  output: 'src/openapi',
});
