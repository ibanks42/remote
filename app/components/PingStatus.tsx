import { useIsFocused } from '@react-navigation/native';
import { useQuery } from '@tanstack/react-query';
import React from 'react';
import { z } from 'zod';
import { Text } from '~/components/ui/text';
import { storage } from '~/lib/storage';

const pingSchema = z.string().refine((value) => value === 'pong');

export function PingStatus() {
	const focused = useIsFocused();

	storage.addOnValueChangedListener((key) => {
		if (key === 'address' || key === 'port') {
			setConnectionStatus(undefined);
			pingQuery.refetch();
		}
	});
	const [connectionStatus, setConnectionStatus] = React.useState<boolean | undefined>(undefined);

	const pingQuery = useQuery({
		queryKey: ['ping'],
		queryFn: async () => {
			try {
				const address = storage.getString('address');
				const port = storage.getNumber('port');

				console.log('pinging', `http://${address}:${port}/ping`);

				const response = await fetch(`http://${address}:${port}/ping`);
				const parsed = pingSchema.safeParse(await response.text());

				setConnectionStatus(true);
				return parsed.data;
			} catch (e) {
				console.log(e);
				setConnectionStatus(false);
				return null;
			}
		},
		refetchInterval: 5000,
		refetchOnWindowFocus: true,
		refetchIntervalInBackground: false,
		refetchOnMount: true,
		enabled: focused,
	});

	function ConnectionStatus() {
		if (connectionStatus === undefined)
			return <Text className='text-muted-foreground'>Checking...</Text>;

		return connectionStatus ? (
			<Text className='text-green-500'>Connected</Text>
		) : (
			<Text className='text-destructive'>Disconnected</Text>
		);
	}

	return (
		<Text>
			Status: <ConnectionStatus />
		</Text>
	);
}
