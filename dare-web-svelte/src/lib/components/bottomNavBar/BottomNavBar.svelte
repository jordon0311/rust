<script lang="ts">
  import { LightSwitch, TabAnchor, TabGroup } from "@skeletonlabs/skeleton";
  import {
    GithubSolid,
    HomeOutline,
    InfoCircleOutline,
    RocketOutline,
    UploadOutline,
  } from "flowbite-svelte-icons";
  export let path = "/";
</script>

<div class="sm:hidden flex flex-col w-full">
  <TabGroup justify="justify-center">
    <TabAnchor href="/" selected={path === "/"}>
      <HomeOutline size="lg" class="m-auto" />
    </TabAnchor>
    <TabAnchor href="/about" selected={path === "/about"}>
      <InfoCircleOutline size="lg" class="m-auto" />
    </TabAnchor>
    <TabAnchor href="/create" selected={path === "/create"}>
      <UploadOutline size="lg" class="m-auto" />
    </TabAnchor>
    <TabAnchor href="/play" selected={path === "/play"}>
      <RocketOutline size="lg" class="m-auto" />
    </TabAnchor>
    <TabAnchor href="/" target="_blank" title="Account">
      <GithubSolid size="lg" class="m-auto" />
    </TabAnchor>
    <TabAnchor>
      <LightSwitch class="m-auto" />
    </TabAnchor>
  </TabGroup>
</div>
