import { createOpenAPI } from 'fumadocs-openapi/server';

export const openapi = createOpenAPI({
  input: ["./lib/openapi.yml"],
  generateCodeSamples(endpoint) {
    return [
      {
        lang: 'curl',
        label: 'cURL',
        source: false,
      },
      {
        lang: 'javascript',
        label: 'JavaScript',
        source: false,
      },
      {
        lang: 'go',
        label: 'Go',
        source: false,
      },
      {
        lang: 'python',
        label: 'Python',
        source: false,
      },
      {
        lang: 'java',
        label: 'Java',
        source: false,
      },
      {
        lang: 'c#',
        label: 'C#',
        source: false,
      },
    ];
  },
});