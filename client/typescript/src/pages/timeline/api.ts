import Http from '../../utils/http';

import { Post } from '../../models';

function getPosts(): Promise<Post[]> {
  const url = `${Http.baseUrl}/posts`;
  return Http.get<Post[]>(url);
}

export { getPosts };
