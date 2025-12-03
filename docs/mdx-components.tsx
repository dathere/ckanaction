import defaultComponents from "fumadocs-ui/mdx";
import type { MDXComponents } from "mdx/types";
import { APIPage } from "@/components/api-page";

// make sure you can use it in MDX files
export function getMDXComponents(components?: MDXComponents): MDXComponents {
  return {
    ...defaultComponents,
    APIPage,
    ...components,
  };
}
