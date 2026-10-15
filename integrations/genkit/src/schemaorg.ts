import { z } from "zod";

export interface SchemaOrgOutputOptions {
  includeContext?: boolean;
}

export function schemaOrgOutput(typeName: string, options: SchemaOrgOutputOptions = {}) {
  const schema = z.object({
    "@context": z.string().optional().default("https://schema.org"),
    "@type": z.string().optional().default(typeName),
    name: z.string().optional(),
    description: z.string().optional(),
    url: z.string().optional(),
  }).passthrough();

  return {
    typeName,
    schema,
    hydrate: async (data: Record<string, any>) => {
      const copy = { ...data };
      if (!copy["@context"] && options.includeContext !== false) {
        copy["@context"] = "https://schema.org";
      }
      if (!copy["@type"]) {
        copy["@type"] = typeName;
      }
      return copy;
    },
    validate: async (data: Record<string, any>) => {
      const result = schema.safeParse(data);
      return {
        isValid: result.success,
        diagnostics: result.success ? [] : result.error.issues,
      };
    },
  };
}
