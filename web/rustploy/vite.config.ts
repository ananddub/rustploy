import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import generouted from '@generouted/react-router/plugin';
import tailwindcss from '@tailwindcss/vite';
import path from 'path';

export default defineConfig({
	plugins: [
		tailwindcss(),
		react(),
		generouted()
	],
	resolve: {
		alias: {
			'@': path.resolve(__dirname, './src'),
			'$lib': path.resolve(__dirname, './src/lib')
		}
	}
});
