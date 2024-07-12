import * as React from 'react';
import { View } from 'react-native';
import { Button } from '~/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '~/components/ui/card';
import { Play, VolumeMinus, VolumePlus } from '~/lib/icons';

export default function Screen() {
  async function callApi(api: 'pause' | 'volume-up' | 'volume-down') {
    try {
      const response = await fetch(`http://10.0.2.2:6969/api/mpv/${api}`);
      console.log(await response.text());
    } catch (e) {
      console.log(e);
    }
  }

  return (
    <View className='flex-1 justify-center items-center gap-5 p-6 bg-secondary/30'>
      <Card className='w-full max-w-sm p-6 rounded-2xl'>
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
                <Play className='text-foreground' size={20} />
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
            <View className='items-center'>
              <Button
                variant='outline'
                className='shadow shadow-foreground/5'
                onPress={() => callApi('volume-down')}
              >
                <VolumeMinus className='text-foreground' size={20} />
              </Button>
            </View>
          </View>
        </CardContent>
      </Card>
    </View>
  );
}
