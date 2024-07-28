import React from 'react';
import { Pressable, View } from 'react-native';
import { Menu } from '~/lib/icons/Menu';
import { cn } from '~/lib/utils';

export function MenuToggle({
	open,
	onChange,
}: { open?: boolean; onChange?: (open: boolean) => void }) {
	return (
		<>
			<Pressable
				onPress={() => onChange?.(!open)}
				className='web:ring-offset-background web:transition-colors web:focus-visible:outline-none web:focus-visible:ring-2 web:focus-visible:ring-ring web:focus-visible:ring-offset-2 mr-2'
			>
				{({ pressed }) => (
					<View
						className={cn(
							'flex-1 aspect-square pt-0.5 justify-center items-start web:px-5',
							pressed && 'opacity-0',
						)}
					>
						<Menu className='text-foreground' size={26} strokeWidth={1.25} />
					</View>
				)}
			</Pressable>
		</>
	);
}
