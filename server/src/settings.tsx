import { zodResolver } from '@hookform/resolvers/zod';
import { invoke } from '@tauri-apps/api/core';
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { Info } from 'lucide-react';
import React from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { Button } from './components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from './components/ui/card';
import { Checkbox } from './components/ui/checkbox';
import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
} from './components/ui/form';
import { Input } from './components/ui/input';
import { Tooltip, TooltipContent, TooltipTrigger } from './components/ui/tooltip';

const schema = z.object({
	port: z.number().int().positive().default(7400),
	mpv: z.object({
		pipe: z.string(),
	}),
	autohide: z.boolean().optional().default(true),
});
export default function SettingsPage() {
	const form = useForm<z.infer<typeof schema>>({
		resolver: zodResolver(schema),
		defaultValues: {
			port: 7920,
			mpv: {
				pipe: '\\\\.\\pipe\\mpvpipe',
			},
			autohide: false,
		},
	});

	const [autostartEnabled, setAutostartEnabled] = React.useState(false);

	React.useEffect(() => {
		async function init() {
			try {
				const settings = schema.safeParse(await invoke('load_settings'));

				if (settings.success && settings.data) {
					form.reset(settings.data);
				} else {
					console.error(settings.error);
				}

				setAutostartEnabled(await isEnabled());
			} catch (e) {
				console.error(e);
			}
		}

		init();
	}, []);

	async function saveSettings() {
		await invoke('save_settings', { settings: JSON.stringify(form.getValues()) });

		if (autostartEnabled) {
			await enable();
		} else {
			await disable();
		}
	}

	return (
		<Form {...form}>
			<div className='flex flex-col text-start gap-2 pb-12'>
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

				<Card>
					<CardHeader>
						<CardTitle>Startup</CardTitle>
					</CardHeader>
					<CardContent className='flex flex-col gap-4'>
						<div className='flex flex-row items-center space-x-2 space-y-0 rounded-md'>
							<FormControl>
								<Checkbox
									checked={autostartEnabled}
									onCheckedChange={(val) => setAutostartEnabled(val as boolean)}
								/>
							</FormControl>
							<FormLabel>Autostart</FormLabel>
						</div>
						<FormField
							control={form.control}
							name='autohide'
							render={({ field }) => (
								<FormItem className='flex flex-row items-center space-x-2 space-y-0 rounded-md'>
									<FormControl>
										<Checkbox
											checked={field.value}
											onCheckedChange={(val) => field.onChange(val as boolean)}
										/>
									</FormControl>

									<FormLabel className='flex items-center gap-2'>Visible on Startup</FormLabel>
								</FormItem>
							)}
						/>
					</CardContent>
				</Card>

				<Button onClick={saveSettings}>Save Settings</Button>
			</div>
		</Form>
	);
}
