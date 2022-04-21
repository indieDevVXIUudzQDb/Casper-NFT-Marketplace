import { supabaseServerSideClient } from "../../utils/supabaseServerSideClient";
import { NFTMeta } from "../../utils/types";

export default async function handler(req: Request, res: Response) {
  if (req.method === "POST") {
    console.log("hello POST");

    const item = (await req.json()) as NFTMeta;

    // supabaseServerSideClient.from("item").insert();
    // TODO return deploy hash

    const { error } = await supabaseServerSideClient
      .from("item")
      .insert({ item });
  }
  // @ts-ignore
  res.status(200);
}
