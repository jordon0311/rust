<script lang="ts">
  import { AppRail, AppRailAnchor, LightSwitch } from "@skeletonlabs/skeleton";
  import {
    GithubSolid,
    HomeOutline,
    InfoCircleOutline,
    RocketOutline,
    UploadOutline,
  } from "flowbite-svelte-icons";
  export let path = "/";
</script>

<div class="hidden sm:flex h-full">
  <AppRail>
    <svelte:fragment slot="lead">
      <AppRailAnchor href="/" selected={path === "/"}>
        <HomeOutline size="xl" class="m-auto" />
      </AppRailAnchor>
    </svelte:fragment>
    <!-- --- -->
    <AppRailAnchor href="/about" selected={path === "/about"}>
      <InfoCircleOutline size="xl" class="m-auto" />
    </AppRailAnchor>
    <AppRailAnchor href="/create" selected={path === "/create"}>
      <UploadOutline size="xl" class="m-auto" />
    </AppRailAnchor>
    <AppRailAnchor href="/play" selected={path === "/play"}>
      <RocketOutline size="xl" class="m-auto" />
    </AppRailAnchor>
    <!-- --- -->
    <svelte:fragment slot="trail">
      <AppRailAnchor href="/" target="_blank" title="Account">
        <GithubSolid size="xl" class="m-auto" />
      </AppRailAnchor>
      <AppRailAnchor>
        <LightSwitch class="m-auto" />
      </AppRailAnchor>
    </svelte:fragment>
  </AppRail>
</div>
