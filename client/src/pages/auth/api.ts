import Http from '../../utils/http';
import { Session } from '../../models';

interface LoginBody {
  email: string;
  password: string;
}

function login(email: string, password: string): Promise<Session> {
  const url = 'http://127.0.0.1:8080/auth/login';
  const body: LoginBody = {
    email,
    password,
  };

  return Http.post<LoginBody, Session>(url, body);
}

export { login };
