<script>
	export let data;
</script>

<p>blog</p>
<ol>
	{#each data.summaries as { slug, title }}
		<li><a href="/blog/{slug}">{title}</a></li>
	{/each}
</ol>
