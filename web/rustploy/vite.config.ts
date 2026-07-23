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
	},
	build: {
		chunkSizeWarningLimit: 600,
		rollupOptions: {
			output: {
				manualChunks(id) {
					if (id.includes('node_modules')) {
						if (id.includes('/react/') || id.includes('/react-dom/') || id.includes('/react-router/') || id.includes('/react-router-dom/')) {
							return 'vendor-react';
						}
						if (id.includes('lucide-react')) {
							return 'vendor-lucide';
						}
						if (id.includes('framer-motion') || id.includes('motion-dom') || id.includes('motion-utils')) {
							return 'vendor-motion';
						}
						if (id.includes('@monaco-editor')) {
							return 'vendor-monaco';
						}
						if (id.includes('@radix-ui')) {
							return 'vendor-radix';
						}
						return 'vendor-deps';
					}
				}
			}
		}
	}
});
