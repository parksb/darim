import React, { useEffect, useState } from 'react';

import { Post } from '../../models';
import * as api from './api';
import Item from './Item';

const List: React.FC = () => {
  const [posts, setPosts] = useState<Post[]>([]);

  const load = async () => {
    const post_list = await api.getPosts();
    setPosts(post_list);
  };

  useEffect(() => {
    load();
  }, []);

  return <div>
    {posts.map((post: Post) => {
      return <Item key={post.id} post={post}/>
    })}
  </div>
};

export default List;
