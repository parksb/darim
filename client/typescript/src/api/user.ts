import Http from '../utils/http';

interface CreateUserBody {
  key: string;
  pin: string;
}

async function createUser(key: string, pin: string): Promise<boolean | null> {
  const url = `${Http.baseUrl}/users`;

  const body: CreateUserBody = {
    key,
    pin,
  };

  try {
    return await Http.post<CreateUserBody, boolean>(url, body);
  } catch (e) {
    alert('Failed to verify email');
  }

  return null;
}

export { createUser };
