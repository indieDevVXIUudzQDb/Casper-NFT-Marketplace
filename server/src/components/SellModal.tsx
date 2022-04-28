import { Button, Group, Modal, TextInput } from "@mantine/core";
import { useForm } from "@mantine/hooks";
import { NFT } from "../utils/cep47_utils";

interface Props {
  opened: boolean;
  setOpened: (o: boolean) => void;
  item: NFT;
  onApproveClick: () => void;
  onSellClick: (item: NFT, amount: string) => void;
}
export function SellModal(props: Props) {
  const { item, opened, setOpened, onSellClick } = props;
  const form = useForm({
    initialValues: {
      amount: "",
    },
  });
  // TODO check for approved
  return (
    <Modal
      opened={opened}
      centered={true}
      onClose={() => setOpened(false)}
      title="Sell NFT"
    >
      {/*  TODO get approved status */}
      <form
        onSubmit={form.onSubmit((values) => {
          if (values.amount) {
            onSellClick(item, values.amount);
            setOpened(false);
          }
        })}
      >
        <Group grow={true} className={"p-2"}>
          <TextInput
            required={true}
            label="Price"
            type={"number"}
            placeholder={"1000000"}
            {...form.getInputProps("amount")}
          />
        </Group>
        <Group position="center" className={"pt-2"}>
          <Group position="right" mt="md">
            <Button type="submit">Sell</Button>
          </Group>
        </Group>
      </form>
    </Modal>
  );
}
