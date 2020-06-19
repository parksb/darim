import Http from '../../utils/http';
import Post from "../../models/Post";

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

function createPost(title: string, date: string, content: string): Promise<number> {
  const url = 'http://127.0.0.1:8080/posts';
  const body: CreatePostBody = {
    title,
    date,
    content,
  };

  return Http.post<CreatePostBody, number>(url, body);
}

function updatePost(id: number, title?: string, date?: string, content?: string): Promise<boolean> {
  if (!title && !date && !content) {
    return Promise.resolve(false);
  }

  const url = `http://127.0.0.1:8080/posts/${id}`;
  const body: UpdatePostBody = {
    title,
    date,
    content,
  };

  return Http.patch<UpdatePostBody, boolean>(url, body);
}

function fetchPost(id: number): Promise<Post> {
  const url = `http://127.0.0.1:8080/posts/${id}`;
  return Http.get<Post>(url);
}

export { createPost, updatePost, fetchPost };
