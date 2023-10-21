import { type Metadata } from "next";
import tailwindConfigFile from "tailwind.config";
import resolveConfig from "tailwindcss/resolveConfig";

export { type Metadata } from "next";

const tailwindConfig = resolveConfig(tailwindConfigFile);

const titleTemplate = {
  template: "%s | Anime Watcher",
  default: "Anime Watcher",
} satisfies Metadata["title"];

const description =
  "Watches for new anime episodes and handles metadata for them." satisfies Metadata["description"];

export const BASE_METADATA = {
  metadataBase: new URL("https://anime-watcher.tethys.ji0.li/"),
  title: titleTemplate,
  description,
  openGraph: {
    type: "website",
    locale: "en_US",
    url: "/",
    siteName: "Anime Watcher",
    title: titleTemplate,
    description,
  },
  twitter: {
    card: "summary_large_image",
    title: titleTemplate,
    description,
  },
  alternates: {
    canonical: "/",
  },
  colorScheme: "dark",
  applicationName: "Anime Watcher",
  referrer: "origin-when-cross-origin",
  robots: {
    index: true,
    follow: true,
    nocache: false,
    nosnippet: false,
    noimageindex: false,
  },
  icons: {
    icon: "/favicon.ico",
  },
  themeColor: (
    tailwindConfig.theme as unknown as { colors: Record<string, string> }
  ).colors.primary,
  viewport: {
    width: "device-width",
    initialScale: 1,
  },
  appleWebApp: {
    statusBarStyle: "black-translucent",
  },
} satisfies Metadata;

export type TemplateString = Exclude<NonNullable<Metadata["title"]>, string>;

export type MetaImageObj = {
  url: string | URL;
  secureUrl?: string | URL;
  alt?: string;
  type?: string;
  width?: string | number;
  height?: string | number;
};

export type MetadataImage = string | MetaImageObj | URL;

export type BasicMetadata = {
  title: string;
  description?: string;
  image?: MetadataImage | MetadataImage[];
};

export const $metadata = (metadata: BasicMetadata): Metadata => {
  const base = {
    ...BASE_METADATA,
    title: {
      ...BASE_METADATA.title,
    },
    openGraph: {
      ...BASE_METADATA.openGraph,
    },
    twitter: {
      ...BASE_METADATA.twitter,
    },
  } as Metadata;

  if (metadata.title) {
    base.title = metadata.title;
    base.openGraph!.title = metadata.title;
    base.twitter!.title = metadata.title;
  }

  if (metadata.description) {
    base.description = metadata.description;
    base.openGraph!.description = metadata.description;
    base.twitter!.description = metadata.description;
  }

  if (metadata.image) {
    base.openGraph!.images = metadata.image;
    base.twitter!.images = metadata.image;
  }

  return {
    ...BASE_METADATA,
    title: {
      ...BASE_METADATA.title,
      default: metadata.title,
    },
    description: metadata.description ?? BASE_METADATA.description,
    openGraph: {
      ...BASE_METADATA.openGraph,
      title: metadata.title,
      description: metadata.description ?? BASE_METADATA.description,
    },
    twitter: {
      ...BASE_METADATA.twitter,
      title: metadata.title,
      description: metadata.description ?? BASE_METADATA.description,
    },
  };
};
