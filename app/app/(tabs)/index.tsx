import * as React from 'react';
import { View } from 'react-native';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '~/components/ui/card';
import { Pause, Play, VolumeMinus, VolumePlus } from '~/lib/icons';
import { Text } from '~/components/ui/text';
import { useQuery } from '@tanstack/react-query';
import { z } from 'zod';
import { useIsFocused } from '@react-navigation/native';
import { storage } from '~/lib/storage';

const schema = z.object({
	volume: z.number(),
	paused: z.boolean(),
	length: z.number(),
	position: z.number(),
	title: z.string(),
	file: z.string(),
});

export default function Screen() {
	const focused = useIsFocused();
	const [status, setStatus] = React.useState<z.infer<typeof schema> | undefined>(undefined);

	async function callApi(api: 'pause' | 'volume-up' | 'volume-down') {
		try {
			await fetch(`http://${storage.getString('address')}:${storage.getNumber('port')}/mpv/${api}`);
			refetch();
		} catch (e) {
			console.log(e);
		}
	}

	const { refetch } = useQuery({
		queryKey: ['mpv-status'],
		queryFn: async () => {
			try {
				const address = storage.getString('address');
				const port = storage.getNumber('port');
				const response = await fetch(`http://${address}:${port}/mpv/status`);

				const parsed = schema.safeParse(await response.json());

				if (parsed.data) setStatus(parsed.data);

				return parsed.data;
			} catch (e) {
				console.log('[MPV Status]', e);
				setStatus(undefined);
				return null;
			}
		},
		refetchInterval: 1000,
		refetchIntervalInBackground: false,
		refetchOnWindowFocus: true,
		enabled: focused,
	});

	function toReadableTime(time: number | undefined) {
		if (time === undefined) {
			return '-';
		}

		// Hours, minutes and seconds
		const hours = Math.floor(time / 3600);
		const minutes = Math.floor((time % 3600) / 60);
		const seconds = Math.floor(time % 60);

		let result = '';

		if (hours > 0) {
			result += `${hours}:${minutes < 10 ? '0' : ''}`;
		}

		result += `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;

		return result;
	}

	if (!focused) return <></>;

	return (
		<View className='flex-1 items-center gap-5 p-6 bg-secondary/30'>
			<Card className='w-full max-w-md p-6 rounded-2xl'>
				<CardHeader className='items-center'>
					<CardTitle className='pb-2 text-center'>MPV Controls</CardTitle>
				</CardHeader>

				<CardContent>
					<View className='flex-row justify-around gap-3'>
						<View className='items-center'>
							<Button
								variant='outline'
								className='shadow shadow-foreground/5'
								onPress={() => callApi('pause')}
							>
								{status?.paused ? (
									<Play className='text-foreground' size={20} />
								) : (
									<Pause className='text-foreground' size={20} />
								)}
							</Button>
						</View>

						<View className='items-center'>
							<Button
								variant='outline'
								className='shadow shadow-foreground/5'
								onPress={() => callApi('volume-down')}
							>
								<VolumeMinus className='text-foreground' size={20} />
							</Button>
						</View>

						<View className='items-center'>
							<Button
								variant='outline'
								className='shadow shadow-foreground/5'
								onPress={() => callApi('volume-up')}
							>
								<VolumePlus className='text-foreground' size={20} />
							</Button>
						</View>
					</View>
				</CardContent>

				<CardFooter>
					<View className='flex-col text-start'>
						<Text className='text-muted-foreground'>Volume: {status?.volume}</Text>
						<Text className='text-muted-foreground'>
							{toReadableTime(status?.position)}/{toReadableTime(status?.length)}
						</Text>
						<Text className='text-muted-foreground'>{status?.title?.replaceAll(`"`, '')}</Text>
					</View>
				</CardFooter>
			</Card>
		</View>
	);
}
