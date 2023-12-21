<script lang="ts">
  import { page } from "$app/stores";
  import BottomNavBar from "$lib/components/bottomNavBar/BottomNavBar.svelte";
  import SideNavBar from "$lib/components/sideNavBar/SideNavBar.svelte";
  import { AppShell } from "@skeletonlabs/skeleton";
  import "highlight.js/styles/github-dark.css";
  import "../app.postcss";
</script>

<!-- App Shell -->
<AppShell>
  <svelte:fragment slot="sidebarLeft">
    <SideNavBar path={$page.url.pathname} />
  </svelte:fragment>
  <svelte:fragment slot="pageFooter">
    <BottomNavBar path={$page.url.pathname} />
  </svelte:fragment>
  <!-- Page Route Content -->
  <slot />
</AppShell>
