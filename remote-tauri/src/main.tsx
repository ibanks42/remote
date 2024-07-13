import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { ThemeProvider } from './components/theme-provider';
import { TooltipProvider } from './components/ui/tooltip';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<ThemeProvider defaultTheme='dark' storageKey='theme'>
			<TooltipProvider>
				<App />
			</TooltipProvider>
		</ThemeProvider>
	</React.StrictMode>,
);
