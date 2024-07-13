import { useForm } from 'react-hook-form';
import { Button } from './components/ui/button';
import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
} from './components/ui/form';
import { Input } from './components/ui/input';
import { invoke } from '@tauri-apps/api/tauri';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from './components/ui/card';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './components/ui/tooltip';
import { TooltipArrow } from '@radix-ui/react-tooltip';
import { Info } from 'lucide-react';

const schema = z.object({
	port: z.number().int().positive().default(7400),
	mpv: z.object({
		pipe: z.string(),
	}),
});
export default function SettingsPage() {
	const form = useForm<z.infer<typeof schema>>({
		resolver: zodResolver(schema),
		defaultValues: {
			port: 7920,
			mpv: {
				pipe: '\\\\.\\pipe\\mpvpipe',
			},
		},
	});

	React.useEffect(() => {
		invoke('load_settings').then((settings) => {
			const parsed = schema.safeParse(settings);
			if (parsed.success && parsed.data) {
				form.reset(parsed.data);
			}
		});
	}, []);

	async function saveSettings() {
		await invoke('save_settings', { settings: JSON.stringify(form.getValues()) });
	}
	return (
		<>
			<Form {...form}>
				<div className='flex flex-col text-start gap-4 pb-12'>
					<h1 className='text-3xl font-bold'>Settings</h1>

					<Card>
						<CardHeader>
							<CardTitle>Server</CardTitle>
						</CardHeader>
						<CardContent>
							<FormField
								control={form.control}
								name='port'
								render={({ field }) => (
									<FormItem>
										<FormLabel>Port</FormLabel>
										<FormControl>
											<Input {...field} onChange={(e) => field.onChange(Number(e.target.value))} />
										</FormControl>
										<FormDescription>Port to be used for the mobile app connection</FormDescription>
									</FormItem>
								)}
							/>
						</CardContent>
					</Card>

					<Card>
						<CardHeader>
							<CardTitle>MPV</CardTitle>
						</CardHeader>
						<CardContent>
							<FormField
								control={form.control}
								name='mpv.pipe'
								render={({ field }) => (
									<FormItem>
										<FormLabel className='flex items-center gap-2'>
											Pipe
											<Tooltip>
												<TooltipTrigger asChild>
													<Info className='h-4 w-4' />
												</TooltipTrigger>

												<TooltipContent>
													<p className='pb-2'>Pipe used for communicating with mpv</p>

													<p>The pipe must be in the format of:</p>

													<ul className='list-disc list-inside'>
														<li>\\.\pipe\mpvpipe</li>
														<li>Where 'mpvpipe' is the name of the pipe</li>
													</ul>
												</TooltipContent>
											</Tooltip>
										</FormLabel>

										<FormControl>
											<Input {...field} />
										</FormControl>

										<FormDescription>Pipe used for communicating with mpv</FormDescription>
									</FormItem>
								)}
							/>
						</CardContent>
					</Card>

					<Button onClick={saveSettings}>Save Settings</Button>
				</div>
			</Form>
		</>
	);
}
