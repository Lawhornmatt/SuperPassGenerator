import { join } from 'path'
import type { Config } from 'tailwindcss'
import forms from '@tailwindcss/forms';
import { skeleton } from '@skeletonlabs/tw-plugin'

export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}', join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')],
	theme: {
		extend: {
			colors: {
				cream:   {
					DEFAULT:  'rgb(255, 250, 240)'
				},
			}
		},
	},
	plugins: [
		forms,
		skeleton({
			themes: {
				preset: [
					{
						name: 'vintage',
						enhancements: true,
					},
					{
						name: 'gold-nouveau',
						enhancements: true,
					},
				],
			},
		}),
	],
} satisfies Config;
