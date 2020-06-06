import Http from '../utils/http';
import { Session } from '../models';

function fetchSession(): Promise<Session> {
  const url = 'http://127.0.0.1:8080/auth';
  return Http.get<Session>(url);
}

export { fetchSession };
