import Http from '../../utils/http';

import { Post } from '../../models';

function getPosts(): Promise<Post[]> {
  const url = 'http://127.0.0.1:8080/posts';
  return Http.get<Post[]>(url);
}

export { getPosts };
