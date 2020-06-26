import SHA3 from 'crypto-js/sha3';

import Http from '../utils/http';
import { Session } from '../models';

interface LoginBody {
  email: string;
  password: string;
}

interface SetSignUpTokenBody {
  name: string;
  email: string;
  password: string;
  avatar_url: string | null;
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

function setSignUpToken(name: string, email: string, password: string, avatarUrl?: string): Promise<boolean> {
  const url = `${Http.baseUrl}/auth/token`;
  const hashedPassword = SHA3(password, { outputLength: 512 }).toString();

  const body: SetSignUpTokenBody = {
    name,
    email,
    password: hashedPassword,
    avatar_url: avatarUrl || null,
  };

  return Http.post<SetSignUpTokenBody, boolean>(url, body);
}

export { fetchSession, login, logout, setSignUpToken };
