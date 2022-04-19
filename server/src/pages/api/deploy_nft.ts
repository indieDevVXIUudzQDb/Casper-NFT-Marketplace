export default function handler(req: Request, res: Response) {
  if (req.method === "POST") {
    console.log("hello POST");
  }
  console.log(req);
  // supabaseServerSideClient.from("item").insert();
  // TODO return deploy hash
  // @ts-ignore
  res.status(200).json(req.body);
}
