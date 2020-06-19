import Http from '../utils/http';
import { Session } from '../models';

function fetchSession(): Promise<Session> {
  const url = `${Http.baseUrl}/auth`;
  return Http.get<Session>(url);
}

export { fetchSession };
