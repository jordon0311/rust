import type { PageLoad } from "./$types";
export const load: PageLoad = async ({ fetch }) => {
  const optionsBuilder = {
    headers: new Headers(),
  };
  optionsBuilder.headers.append("content-type", "application/json");
  const rawResponse: Response = await fetch(
    new Request("http://127.0.0.1:8000", {
      method: "GET",
      keepalive: true,
      mode: "cors",
      credentials: "include",
      ...optionsBuilder,
    })
  );
  console.log({ rawResponse });
  const { ok, status, redirected, type, url } = rawResponse;
  console.log({ ok, status, redirected, type, url });
  const blah = await rawResponse.text();
  console.log({ blah });
  const items = {};
  return { items };
};
