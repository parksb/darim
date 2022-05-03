interface ActiveUserSession {
  is_mine: boolean;
  token_uuid: string;
  user_agent?: string;
  last_accessed_at: number;
}

export default ActiveUserSession;
