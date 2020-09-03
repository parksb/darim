interface Session {
  user_id: string;
  user_email: string;
  user_name: string;
  user_public_key: string;
  user_avatar_url: string | null;
}

export default Session;
