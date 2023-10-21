import Image from "next/image";
import Link from "next/link";

import ImgLogoTextGrayscale from "~/assets/images/logo/logo-text.grayscale.inverted.x48.png";

export default function DefaultLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="bg-background flex min-h-screen flex-col bg-gradient-to-br from-[#2e026d] to-[#15162c]">
      <header className="container mt-12 flex h-12 flex-col">
        <Link className="mx-auto h-full w-auto" href="/">
          <Image
            alt="Anime Watcher logo"
            className="h-full w-auto"
            src={ImgLogoTextGrayscale}
          />
        </Link>
      </header>
      <main className="container flex flex-col pt-12">{children}</main>
      <footer className="mt-auto">
        <div className="container mt-12 rounded-t-lg bg-black p-4 text-gray-300">
          Copyright &copy; 2023{" "}
          <a
            href="https://github.com/allypost"
            rel="noopener noreferrer"
            target="_blank"
          >
            Allypost
          </a>
        </div>
      </footer>
    </div>
  );
}
