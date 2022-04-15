import { createClient } from "@supabase/supabase-js";

const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
//CAUTION: DO NOT USE THIS CLIENT SIDE
const supabaseServiceKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

export const supabaseClientSideClient = createClient(
  // @ts-ignore
  supabaseUrl,
  supabaseServiceKey
);
