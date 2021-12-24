interface User {
  id: string;
  email: string;
  name: string;
  public_key: string;
  avatar_url: string | null;
}

export default User;
