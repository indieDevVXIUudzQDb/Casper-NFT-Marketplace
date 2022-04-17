export default function handler(req, res) {
  console.log(req);
  // supabaseServerSideClient.from("item").insert();
  res.status(200).json({ name: "John Doe" });
}
