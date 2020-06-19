import React, { useEffect, useState } from 'react';

import * as api from './api';
import Item from './Item';
import { Post } from '../../models';
import { Section } from "../../components";

const List: React.FC = () => {
  const [posts, setPosts] = useState<Post[]>([]);

  const load = async () => {
    const post_list = await api.getPosts();
    setPosts(post_list);
  };

  useEffect(() => {
    load();
  }, []);

  return <Section>
    {posts.map((post: Post) => {
      return <Item key={post.id} post={post}/>
    })}
  </Section>
};

export default List;
