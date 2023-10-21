"use client";

import {
  createTheme,
  type FieldsetProps,
  MantineProvider,
  type PopoverProps,
} from "@mantine/core";
import { ModalsProvider } from "@mantine/modals";
import { Notifications } from "@mantine/notifications";
import { NavigationProgress } from "@mantine/nprogress";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { type FC, type PropsWithChildren } from "react";

import { commitMono } from "~/assets/font";

const theme = createTheme({
  fontFamily: commitMono.style.fontFamily,
  fontFamilyMonospace: commitMono.style.fontFamily,
  primaryColor: "violet",
  respectReducedMotion: true,
  cursorType: "pointer",
  black: "#000",
  primaryShade: 9,
  components: {
    Popover: {
      defaultProps: {
        withArrow: true,
        arrowSize: 12,
        position: "bottom-start",
      } satisfies PopoverProps,
    },
    Fieldset: {
      defaultProps: {
        variant: "unstyled",
      } satisfies FieldsetProps,
    },
  },
});

const queryClient = new QueryClient();

export const Providers: FC<PropsWithChildren> = ({ children }) => {
  return (
    <QueryClientProvider client={queryClient}>
      <ReactQueryDevtools buttonPosition="bottom-left" />
      <MantineProvider defaultColorScheme="dark" theme={theme}>
        <Notifications />
        <NavigationProgress />
        <ModalsProvider>{children}</ModalsProvider>
      </MantineProvider>
    </QueryClientProvider>
  );
};
