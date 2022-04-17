import { createClient } from "@supabase/supabase-js";

const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
// CAUTION: DO NOT USE THIS CLIENT SIDE
const supabaseServiceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;

export const supabaseServerSideClient = createClient(
  // @ts-ignore
  supabaseUrl,
  supabaseServiceKey
);
