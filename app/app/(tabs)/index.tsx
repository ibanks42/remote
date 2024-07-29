import { useIsFocused } from '@react-navigation/native';
import { useQuery } from '@tanstack/react-query';
import * as React from 'react';
import { ScrollView, View } from 'react-native';
import { z } from 'zod';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '~/components/ui/card';
import {
	type Option,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '~/components/ui/select';
import { Text } from '~/components/ui/text';
import { Pause, Play, VolumeMinus, VolumePlus } from '~/lib/icons';
import { storage } from '~/lib/storage';

const schema = z.object({
	volume: z.number(),
	paused: z.boolean(),
	length: z.number(),
	position: z.number(),
	title: z.string(),
	file: z.string(),
	subtitle: z.string(),
	subtitles: z.array(z.object({ id: z.number(), title: z.string() })),
});

export default function HomePage() {
	const focused = useIsFocused();
	const [subtitle, setSubtitle] = React.useState<Option | undefined>(undefined);
	const [status, setStatus] = React.useState<z.infer<typeof schema> | undefined>(undefined);

	async function callApi(
		api: 'pause' | 'volume-up' | 'volume-down' | 'subtitle',
		params?: { id: string; value: string }[],
	) {
		try {
			const url = new URL(
				`http://${storage.getString('address')}:${storage.getNumber('port')}/mpv/${api}`,
			);
			if (params) {
				for (const param of params) {
					url.searchParams.append(param.id, param.value);
				}
			}
			console.log(url);

			await fetch(url);
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
				return {};
			}
		},
		refetchInterval: 1000,
		refetchIntervalInBackground: false,
		refetchOnWindowFocus: true,
		enabled: focused,
	});

	React.useEffect(() => {
		if (status?.subtitle) {
			const value = status?.subtitles?.find((s) => s.id.toString() === status?.subtitle);
			setSubtitle({ label: value?.title || 'None', value: status?.subtitle });
		}
	}, [status?.subtitle]);

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

	const subtitleOptions = React.useMemo(() => {
		return status?.subtitles.map((subtitle) => {
			return {
				label: subtitle.title,
				value: subtitle.id.toString(),
			};
		});
	}, [status?.subtitles]);

	if (!focused) return <></>;

	return (
		<View className='flex-1 items-center gap-5 p-6 bg-secondary/30'>
			<Card className='w-full max-w-md p-6 rounded-2xl'>
				<CardHeader className='items-center'>
					<CardTitle className='pb-2 text-center'>MPV Controls</CardTitle>
				</CardHeader>

				<CardContent className='flex-col gap-6'>
					<View className='flex flex-row justify-between'>
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

					<View>
						<Text className='text-muted-foreground'>Subtitles</Text>
						<Select
							value={subtitle}
							onValueChange={(option) =>
								option && callApi('subtitle', [{ id: 'id', value: option.value }])
							}
						>
							<SelectTrigger>
								<SelectValue placeholder='None' className='text-foreground' />
							</SelectTrigger>
							<SelectContent>
								<ScrollView>
									<SelectItem label='None' value='0' />
									{subtitleOptions?.map((subtitle) => (
										<SelectItem
											key={subtitle.value}
											label={subtitle.label}
											value={subtitle.value}
										/>
									))}
								</ScrollView>
							</SelectContent>
						</Select>
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
