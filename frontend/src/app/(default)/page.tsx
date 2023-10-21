import Link from "next/link";

import { list as animeList } from "~/api/v1/anime";

export default async function HomePage() {
  const data = await animeList();

  if (!data) {
    return (
      <>
        <h1>Error</h1>
        <p>Failed to fetch anime.</p>
      </>
    );
  }

  if (data.body.type === "error") {
    return (
      <>
        <h1>Error</h1>
        <pre>{data?.body.data}</pre>
      </>
    );
  }

  if (data.body.type === "empty") {
    return <h1>Error: Empty response</h1>;
  }

  return (
    <>
      <div className="flex">
        <div className="ml-auto">hello</div>
      </div>
      <div className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
        {data.body.data.map(({ anime }) => {
          return (
            <article
              key={anime.id}
              className="flex flex-col gap-2 rounded border-2 bg-black bg-opacity-30 p-4"
            >
              <h2 className="mb-auto inline-block text-2xl font-bold">
                <Link
                  className="cursor-pointer hover:underline"
                  href={`/anime/${anime.id}`}
                >
                  {anime.name}
                </Link>
              </h2>

              <p className="max-h-[10ex] overflow-y-scroll whitespace-pre-wrap border-l-4 pl-4 leading-snug opacity-80">
                {anime.description}
              </p>

              {anime.malId ? (
                <a
                  className="self-end underline opacity-80 hover:no-underline hover:opacity-100"
                  href={`https://myanimelist.net/anime/${anime.malId}`}
                  rel="noopener noreferrer"
                  target="_blank"
                >
                  MyAnimeList page
                </a>
              ) : null}
            </article>
          );
        })}
        {/* <pre>{JSON.stringify(anime.body.data, null, 4)}</pre> */}
      </div>
    </>
  );
}
