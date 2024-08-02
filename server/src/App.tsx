import { AnimatePresence, motion } from 'framer-motion';
import SettingsPage from './settings';

import './global.css';

function App() {
	return (
		<div className='p-8 flex flex-col justify-center h-[100vh] overflow-visible text-center bg-background'>
			<div className='w-full h-full'>
				<AnimatePresence mode='wait'>
					<motion.div
						key='settings'
						initial={{ x: -100, scale: 0 }}
						animate={{ x: 0, scale: 1 }}
						transition={{ type: 'tween', stiffness: 260, damping: 20, duration: 0.2 }}
					>
						<SettingsPage />
					</motion.div>
				</AnimatePresence>
			</div>
		</div>
	);
}

export default App;
