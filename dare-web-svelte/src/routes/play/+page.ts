import type { PageLoad } from "./$types";

type Dare = {
  id: number;
  title: string;
  description: string;
};

export const load: PageLoad = async ({ fetch }) => {
  const optionsBuilder = {
    headers: new Headers(),
  };
  optionsBuilder.headers.append("content-type", "application/json");
  const rawResponse: Response = await fetch(
    new Request("http://127.0.0.1:8000/dares", {
      method: "GET",
      keepalive: true,
      mode: "cors",
      credentials: "include",
      ...optionsBuilder,
    })
  );
  const dares: Dare[] = await rawResponse.json();
  return { dares };
};
