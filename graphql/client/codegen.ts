import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  // schema: 'https://swapi-graphql.netlify.app/.netlify/functions/index',
  schema: '../server/schema.graphql',
  // documents: ['src/**/*.tsx'],
  // ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    './src/gql/': {
      preset: 'client'
    }
  }
}

export default config
