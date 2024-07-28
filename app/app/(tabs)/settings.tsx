import AsyncStorage from '@react-native-async-storage/async-storage';
import React from 'react';
import { Keyboard, View } from 'react-native';
import { z } from 'zod';
import { PingStatus } from '~/components/PingStatus';
import { Button } from '~/components/ui/button';
import { Card, CardContent } from '~/components/ui/card';
import { Input } from '~/components/ui/input';
import { Text } from '~/components/ui/text';
import { storage } from '~/lib/storage';

const schema = z.object({
	address: z.string(),
	port: z.number(),
});

const DEFAULT_ADDRESS = '192.168.0.9';
const DEFAULT_PORT = 42069;

export default function SettingsPage() {
	const [form, setForm] = React.useState<z.infer<typeof schema> | null>(null);

	React.useEffect(() => {
		(async () => {
			let address = storage.getString('address');
			let port = storage.getNumber('port');
			if (!address) {
				await AsyncStorage.setItem('address', DEFAULT_ADDRESS);
				address = DEFAULT_ADDRESS;
			}

			if (!port) {
				storage.set('port', DEFAULT_PORT);
				port = DEFAULT_PORT;
			}

			setForm({ address, port });
		})();
	}, []);

	function saveSettings() {
		if (form) {
			storage.set('address', form.address);
			storage.set('port', form.port);
			Keyboard.dismiss();
		}
	}

	return (
		<View className='flex-1 items-center gap-5 p-6 bg-secondary/30'>
			<Card className='w-full max-w-md p-6 rounded-2xl'>
				<CardContent className='gap-4'>
					<Text>Settings</Text>

					<Input
						value={form?.address}
						onChangeText={(address) =>
							setForm({ port: form?.port || DEFAULT_PORT, address: address })
						}
						onSubmitEditing={saveSettings}
					/>
					<Input
						inputMode='decimal'
						value={form?.port.toString()}
						onChangeText={(port) =>
							setForm({ port: Number(port), address: form?.address || DEFAULT_ADDRESS })
						}
						onSubmitEditing={saveSettings}
					/>

					<PingStatus />

					<Button onPress={saveSettings}>
						<Text>Save</Text>
					</Button>
				</CardContent>
			</Card>
		</View>
	);
}
