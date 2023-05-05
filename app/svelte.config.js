/** @type {import('@sveltejs/kit').Config} */
import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

const config = {
	preprocess: preprocess(),
	kit: {
		adapter: adapter(),
	},

	vitePlugin: {
		onwarn: (warning, defaultHandler) => {
			console.log('svelte:warnings:%s', JSON.stringify(warning));
			defaultHandler(warning);
		}
	}
};

export default config;
