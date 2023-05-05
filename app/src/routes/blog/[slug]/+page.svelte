<script>
	import Greet from '$lib/Greet.svelte';
	export let data;
</script>

<h1>{data.post.title}</h1>
<div>{@html data.post.content}</div>
<Greet />

