import Http from '../utils/http';
import Post from "../models/Post";

interface CreatePostBody {
  title: string;
  date: string;
  content: string;
}

interface UpdatePostBody {
  title?: string;
  date?: string;
  content?: string;
}

function getPosts(): Promise<Post[]> {
  const url = `${Http.baseUrl}/posts`;
  return Http.get<Post[]>(url);
}

async function createPost(title: string, date: string, content: string): Promise<number | null> {
  const url = `${Http.baseUrl}/posts`;
  const body: CreatePostBody = {
    title,
    date,
    content,
  };

  try {
    return await Http.post<CreatePostBody, number>(url, body);
  } catch (e) {
    alert('Failed to save post');
  }

  return null;
}

async function updatePost(id: number, title?: string, date?: string, content?: string): Promise<boolean | null> {
  if (!title && !date && !content) {
    return Promise.resolve(false);
  }

  const url = `${Http.baseUrl}/posts/${id}`;
  const body: UpdatePostBody = {
    title,
    date,
    content,
  };

  try {
    return await Http.patch<UpdatePostBody, boolean>(url, body);
  } catch (e) {
    alert('Failed to save post')
  }

  return null;
}

function fetchPost(id: number): Promise<Post> {
  const url = `${Http.baseUrl}/posts/${id}`;
  return Http.get<Post>(url);
}

export { getPosts, createPost, updatePost, fetchPost };
