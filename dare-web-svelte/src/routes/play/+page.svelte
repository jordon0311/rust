<script lang="ts">
  import DareGame from "$lib/components/dareGame/DareGame.svelte";
  import type { PageData } from "./$types";
  export let data: PageData;
</script>

<div class="container h-full mx-auto flex justify-center items-center">
  <section class="img-bg" />
  <DareGame dare={data.dares[0]} />
</div>

<style lang="postcss">
  figure {
    @apply flex relative flex-col;
  }
  figure svg,
  .img-bg {
    @apply w-64 h-64 md:w-80 md:h-80;
  }
  .img-bg {
    @apply absolute z-[-1] rounded-full blur-[50px] transition-all;
    animation:
      pulse 5s cubic-bezier(0, 0, 0, 0.5) infinite,
      glow 3s linear infinite;
  }
  @keyframes glow {
    0% {
      @apply bg-primary-400/50;
    }
    33% {
      @apply bg-secondary-400/50;
    }
    66% {
      @apply bg-tertiary-400/50;
    }
    100% {
      @apply bg-primary-400/50;
    }
  }
  @keyframes pulse {
    50% {
      transform: scale(3, 2);
    }
  }
</style>
