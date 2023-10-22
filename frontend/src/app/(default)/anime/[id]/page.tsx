import { info as fetchAnimeInfo } from "~/api/v1/anime";
import { type PageProps } from "~/types/app-router";
import { $metadata } from "~/util/metadata";

import {
  AddNewSourceButton,
  DeleteSourceButton,
  EditAnimeInfoButton,
} from "./page.client";

export const generateMetadata = async (props: PageProps<{ id: string }>) => {
  const data = await fetchAnimeInfo(Number(props.params.id));

  if (!data || data.body.type !== "success") {
    return undefined;
  }

  return $metadata({
    title: data.body.data.anime.name,
  });
};

export default async function PageAnimeIdHome({
  params,
}: PageProps<{ id: string }>) {
  const data = await fetchAnimeInfo(Number(params.id));

  if (!data || data.body.type !== "success") {
    return undefined;
  }

  const { anime, sources } = data.body.data;

  return (
    <article className="grid grid-cols-1 justify-items-center gap-6 lg:grid-cols-[minmax(0,theme(maxWidth.prose)),1fr]">
      <h1 className="text-4xl font-bold tracking-wider">
        <span className="block max-w-prose">{anime.name}</span>
      </h1>

      <div className="self-center">
        <EditAnimeInfoButton key={JSON.stringify(anime)} anime={anime} />
      </div>

      <div className="flex max-w-prose flex-col gap-[inherit] justify-self-stretch">
        <p className="prose prose-invert whitespace-pre-wrap">
          {anime.description}
        </p>

        <div className="mt-auto">
          {anime.malId ? (
            <a
              className="underline opacity-80 hover:no-underline hover:opacity-100"
              href={`https://myanimelist.net/anime/${anime.malId}`}
              rel="noopener noreferrer"
              target="_blank"
            >
              MyAnimeList page
            </a>
          ) : null}
        </div>
      </div>

      <div className="flex flex-col gap-[inherit]">
        <section>
          <h3>Sources:</h3>
          <ul className="relative ml-12 inline-block">
            {sources.map((source) => (
              <li
                key={source.id}
                className="list-[decimal-leading-zero] text-white/50"
              >
                <div className="flex items-center pl-2 text-white">
                  <span>
                    [{source.seriesSite}] {source.seriesSiteId}
                  </span>

                  <span className="ml-auto inline-flex pl-6">
                    <DeleteSourceButton
                      sourceId={source.id}
                      confirmText={
                        <>
                          <p>Are you sure you want to delete this source?</p>
                          <p className="italic">
                            [{source.seriesSite}] {source.seriesSiteId}
                          </p>
                        </>
                      }
                    />
                  </span>
                </div>
              </li>
            ))}
            <li className="list-none">
              <span className="absolute -left-2 bottom-0 -translate-x-full select-none text-white/50">
                /\.
              </span>
              <AddNewSourceButton forAnimeId={anime.id} />
            </li>
          </ul>
        </section>
      </div>
    </article>
  );
}
