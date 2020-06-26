import Http from '../utils/http';

interface CreateUserBody {
  key: string;
  pin: string;
}

function createUser(key: string, pin: string): Promise<boolean> {
  const url = `${Http.baseUrl}/users`;

  const body: CreateUserBody = {
    key,
    pin,
  };

  return Http.post<CreateUserBody, boolean>(url, body);
}

export { createUser };
