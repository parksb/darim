import { serverBaseUrl } from '../constants';

interface ResponseData<T> {
  data: T | null;
  error: string | null;
}

enum HttpMethods {
  GET = 'GET',
  POST = 'POST',
  PATCH = 'PATCH',
  DELETE = 'DELETE',
}

class Http {
  private static async request<T>(
    method: HttpMethods,
    url: string,
    body?: string,
    accessToken?: string,
  ): Promise<T> {
    const rawResponse: Response = await fetch(url, {
      method,
      headers: this.composeHeaders(accessToken),
      body,
      credentials: 'include',
    });

    const response: ResponseData<T> = await rawResponse.json();

    if (!response.data) {
      throw new Error(rawResponse.status.toString());
    }

    return response.data;
  }

  static async get<T>(url: string, accessToken: string): Promise<T> {
    try {
      return await Http.request(HttpMethods.GET, url, undefined, accessToken);
    } catch (e) {
      if (e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return Http.get(url, refreshedAccessToken);
      }
      throw e;
    }
  }

  static async post<T, S>(url: string, body?: T, accessToken?: string): Promise<S> {
    try {
      const jsonBody = body ? JSON.stringify(body) : undefined;
      return await Http.request(HttpMethods.POST, url, jsonBody, accessToken);
    } catch (e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return Http.post(url, body, refreshedAccessToken);
      }
      throw e;
    }
  }

  static async patch<T, S>(url: string, body: T, accessToken: string): Promise<S> {
    try {
      const jsonBody = JSON.stringify(body);
      return await Http.request(HttpMethods.PATCH, url, jsonBody, accessToken);
    } catch (e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return Http.patch(url, body, refreshedAccessToken);
      }
      throw e;
    }
  }

  static async delete<T>(url: string, accessToken?: string): Promise<T> {
    try {
      return await Http.request(HttpMethods.DELETE, url, undefined, accessToken);
    } catch (e) {
      if (accessToken && e instanceof Error && e.message === '401') {
        const refreshedAccessToken = await Http.refreshAccessToken();
        return Http.delete(url, refreshedAccessToken);
      }
      throw e;
    }
  }

  private static async refreshAccessToken(): Promise<string> {
    const url = `${serverBaseUrl}/auth/token/access`;
    return Http.request(HttpMethods.POST, url);
  }

  /* eslint-disable no-undef */
  private static composeHeaders(accessToken?: string): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Credentials': 'true',
    };

    if (accessToken) {
      headers.Authorization = `Bearer ${accessToken}`;
    }

    return headers;
  }
}

export default Http;
