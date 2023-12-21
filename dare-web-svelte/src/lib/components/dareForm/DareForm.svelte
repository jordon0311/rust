<script lang="ts">
</script>

<div class="container p-8">
  <form
    class="card p-4 w-full bg-gradient-to-br variant-outline-primary space-y-2 sm:space-y-4">
    <h1 class="h2">
      <span
        class="bg-gradient-to-br variant-gradient-secondary-tertiary bg-clip-text text-transparent box-decoration-clone"
        >Submit a Dare!</span>
    </h1>
    <label
      class="h4 label card bg-gradient-to-br variant-gradient-secondary-tertiary p-4">
      <span>Title</span>
      <input
        class="input"
        title="Title"
        type="text"
        placeholder="I dare you to..." />
    </label>
    <label
      class="h4 label card bg-gradient-to-br variant-gradient-secondary-tertiary p-4">
      <span>Description</span>
      <textarea
        class="textarea"
        rows="4"
        placeholder="...Dance like a robot for 30 seconds" />
    </label>
    <label
      class="h4 label card bg-gradient-to-br variant-gradient-secondary-tertiary p-4 space-y-4">
      <span>Author</span>
      <input class="input" title="Author" type="text" placeholder="Your Name" />
    </label>
  </form>
</div>
