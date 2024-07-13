import { invoke } from '@tauri-apps/api/tauri';
import { Button } from './components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from './components/ui/card';

export default function HomePage() {
	async function pauseMpv() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		await invoke('handle_pause_cmd');
	}
	async function volumeUp() {
		console.log('volume up cmd');
		await invoke('handle_volume_up_cmd');
	}
	async function volumeDown() {
		alert('volume down cmd');
		await invoke('handle_volume_down_cmd');
	}

	return (
		<div className='flex flex-col gap-2 mx-auto text-center'>
			<Card>
				<CardHeader>
					<CardTitle>MPV Remote</CardTitle>
				</CardHeader>
				<CardContent className='flex flex-col gap-2'>
					<Button type='button' onClick={pauseMpv}>
						Pause/Unpause MPV
					</Button>

					<Button type='button' onClick={volumeUp}>
						Volume Up
					</Button>

					<Button type='button' onClick={volumeDown}>
						Volume Down
					</Button>
				</CardContent>
			</Card>
		</div>
	);
}
