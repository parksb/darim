import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
import { Session } from '../models';

interface LoginBody {
  email: string;
  password: string;
}

function fetchSession(): Promise<Session> {
  const url = `${Http.baseUrl}/auth`;
  return Http.get<Session>(url);
}

function login(email: string, password: string): Promise<Session> {
  const url = `${Http.baseUrl}/auth/login`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: LoginBody = {
    email,
    password: hashedPassword,
  };

  return Http.post<LoginBody, Session>(url, body);
}

function logout(): Promise<boolean> {
  const url = `${Http.baseUrl}/auth/logout`;
  return Http.postWithoutBody<boolean>(url);
}

export { fetchSession, login, logout };
