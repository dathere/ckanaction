import { docs } from "fumadocs-mdx:collections/server";
import { type InferPageType, loader } from "fumadocs-core/source";
import { lucideIconsPlugin } from "fumadocs-core/source/lucide-icons";
import { openapiPlugin } from "fumadocs-openapi/server";
import { openapi } from '@/lib/openapi';

// See https://fumadocs.dev/docs/headless/source-api for more info
export const source = loader({
  docs: docs.toFumadocsSource(),
  openapi: await openapi.staticSource({
    baseDir: "openapi"
  }),
},
  {
    baseUrl: "/docs",
    plugins: [lucideIconsPlugin(), openapiPlugin()],
  });

export function getPageImage(page: InferPageType<typeof source>) {
  const segments = [...page.slugs, "image.png"];

  return {
    segments,
    url: `/og/docs/${segments.join("/")}`,
  };
}

export async function getLLMText(page: InferPageType<typeof source>) {
  const processed = await page.data.getText("processed");

  return `# ${page.data.title} (${page.url})

${processed}`;
}
