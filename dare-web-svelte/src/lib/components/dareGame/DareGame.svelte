<script lang="ts">
  import { ArrowRightSolid } from "flowbite-svelte-icons";
</script>

<div class="container p-8">
  <div class="card p-4 w-full variant-filled-surface">
    <h2 class="h2">
      <span
        class="bg-gradient-to-br variant-gradient-secondary-tertiary bg-clip-text text-transparent box-decoration-clone"
        >Let's Play!</span>
    </h2>
    <div class="label h4 px-4 py-2">
      <p class="h4">I dare you to...</p>
    </div>
    <hr class="!border-t-2" />
    <div class="h4 label p-4">
      <p>...Dance like a robot for 30 seconds!</p>
    </div>
    <hr class="!border-t-2" />
    <div class="flex items-center justify-between h4 label p-4">
      <p>Author: Jordon</p>
      <button type="button" class="btn variant-filled">
        <span>Next</span>
        <ArrowRightSolid />
      </button>
    </div>
  </div>
</div>
