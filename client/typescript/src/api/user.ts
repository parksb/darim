import Http from '../utils/http';

interface CreateUserBody {
  user_public_key: string;
  token_key: string;
  token_pin: string;
}

async function createUser(user_public_key: string, token_key: string, token_pin: string): Promise<boolean | null> {
  const url = `${Http.baseUrl}/users`;

  const body: CreateUserBody = {
    user_public_key,
    token_key,
    token_pin,
  };

  try {
    return await Http.post<CreateUserBody, boolean>(url, body);
  } catch (e) {
    alert('Failed to verify email');
  }

  return null;
}

export { createUser };
