import { AnimatePresence, motion } from 'framer-motion';
import Hamburger from 'hamburger-react';
import React from 'react';
import './global.css';
import HomePage from './home';
import SettingsPage from './settings';

function App() {
	const [settingsOpen, setSettingsOpen] = React.useState(false);

	return (
		<div className='pt-8 px-8 flex flex-col justify-center h-[100vh] overflow-visible text-center bg-background'>
			<div className='fixed top-0 left-0 z-50 px-4 pt-2'>
				<Hamburger size={24} onToggle={() => setSettingsOpen(!settingsOpen)} />
			</div>

			<div className='w-full h-full p-8 overflow-hidden'>
				<AnimatePresence mode='wait'>
					{settingsOpen ? (
						<motion.div
							key='settings'
							initial={{ rotate: 180, scale: 0 }}
							animate={{ rotate: 0, scale: 1 }}
							transition={{ type: 'spring', stiffness: 260, damping: 20, duration: 0.5 }}
							exit={{ rotate: 180, scale: 0, transition: { duration: 0.2 } }}
						>
							<SettingsPage />
						</motion.div>
					) : (
						<motion.div
							key='home'
							animate={{ rotate: 0, scale: 1 }}
							transition={{ type: 'spring', stiffness: 260, damping: 20, duration: 0.5 }}
							exit={{ rotate: 180, scale: 0, transition: { duration: 0.2 } }}
						>
							<HomePage />
						</motion.div>
					)}
				</AnimatePresence>
			</div>
		</div>
	);
}

export default App;
