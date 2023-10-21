import "~/assets/styles/tailwind.css";
import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/nprogress/styles.css";
import "~/assets/styles/globals.css";

import { ColorSchemeScript } from "@mantine/core";

import { commitMono } from "~/assets/font";
import { BASE_METADATA } from "~/util/metadata";

import { Providers } from "./providers";

export const metadata = BASE_METADATA;

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html className="dark" lang="en">
      <head>
        <ColorSchemeScript />
      </head>
      <body
        className={`font-mono text-white antialiased ${commitMono.variable}`}
      >
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
