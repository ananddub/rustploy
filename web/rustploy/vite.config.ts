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
		chunkSizeWarningLimit: 800,
		rollupOptions: {
			output: {
				manualChunks: {
					'vendor-react': ['react', 'react-dom', 'react-router-dom'],
					'vendor-motion': ['framer-motion'],
					'vendor-monaco': ['@monaco-editor/react'],
					'vendor-ui': [
						'@radix-ui/react-avatar',
						'@radix-ui/react-dialog',
						'@radix-ui/react-dropdown-menu',
						'@radix-ui/react-progress',
						'@radix-ui/react-select',
						'@radix-ui/react-slot',
						'@radix-ui/react-switch',
						'@radix-ui/react-tabs',
						'lucide-react',
						'sonner',
						'clsx',
						'tailwind-merge'
					]
				}
			}
		}
	}
});
