import AsyncStorage from '@react-native-async-storage/async-storage';
import React from 'react';
import { Keyboard, View } from 'react-native';
import { Input } from '~/components/ui/input';
import { Text } from '~/components/ui/text';
import { z } from 'zod';
import { Button } from '~/components/ui/button';

const schema = z.object({
  address: z.string(),
  port: z.string(),
});

export default function SettingsPage() {
  const [form, setForm] = React.useState<z.infer<typeof schema> | null>(null);

  React.useEffect(() => {
    (async () => {
      let address = await AsyncStorage.getItem('address');
      let port = await AsyncStorage.getItem('port');
      if (!address) {
        await AsyncStorage.setItem('address', '192.168.0.17');
        address = '192.168.0.17';
      }

      if (!port) {
        await AsyncStorage.setItem('port', '6969');
        port = '6969';
      }
      console.log(address, port);

      setForm({ address, port });
    })();
  }, []);

  function saveSettings() {
    if (form) {
      AsyncStorage.setItem('address', form.address);
      AsyncStorage.setItem('port', form.port);
      Keyboard.dismiss();
    }
  }

  return (
    <View className='flex flex-col p-4 justify-center' style={{ gap: 40 }}>
      <Text>Settings</Text>

      <Input
        value={form?.address}
        onChangeText={(address) => setForm({ port: form?.port || '6969', address: address })}
      />
      <Input
        value={form?.port}
        onChangeText={(port) => setForm({ port: port, address: form?.address || '192.168.0.17' })}
      />

      <Button onPress={saveSettings}>
        <Text>Save</Text>
      </Button>
    </View>
  );
}
