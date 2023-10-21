import { z } from "zod";

import { type ApiResponse } from "../types/base";

export type ApiV1Response<T = unknown> = ApiResponse<"v1", T>;

export type Anime = {
  id: number;
  name: string;
  description: string | null;
  createdAt: string;
  updatedAt: string;
  malId: number | null;
};

export type AnimeSource = {
  id: number;
  forSeriesId: number;
  seriesSite: string;
  seriesSiteId: string;
  createdAt: string;
  updatedAt: string;
};

export const animeSourceAddValidation = z.object({
  forSeriesId: z.number(),
  seriesSite: z.string(),
  seriesSiteId: z.string(),
});

export type RespAnimeList = {
  anime: Anime;
  sources: AnimeSource[];
}[];

export type RespAnimeItem = {
  anime: Anime;
  sources: AnimeSource[];
};
