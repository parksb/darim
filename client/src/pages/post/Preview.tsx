import React from 'react';
import styled from 'styled-components';
import MarkdownIt from 'markdown-it';

interface Props {
  content: string;
}

const Content = styled.div`
  max-width: 100%;
  min-height: 720px;
  padding: 5px;
  font-family: sans-serif;
  font-size: 16px;
  line-height: 180%;

  h1, h2, h3, h4, h5, h6 {
    margin-top: 30px;
    margin-bottom: 20px;
    font-weight: 700;
  }

  h1 {
    font-size: 24px;
  }

  h2 {
    font-size: 22px;
  }

  h3 {
    font-size: 20px;
  }

  h4 {
    font-size: 18px;
  }

  h5, h6 {
     font-size: 16px;
  }

  p {
    font-size: 16px;
  }

  a {
    color: #0366d6;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }

  ul, ol {
    padding-left: 20px;
  }

  li {
    list-style-position: inside;
  }

  img {
    max-width: 100%;
  }

  pre {
    background-color: #282a36;
    color: #ffffff;
    padding: 15px 20px 20px 20px;
    margin-top: 20px;
    margin-bottom: 20px;
  }

  code, code span {
    font-family: 'Consolas', 'Monaco', 'Ubuntu Mono', 'Andale Mono', monospace;
  }

  p code, li code {
    background-color: #EEEEEE;
    padding: 0 5px 0 5px;
  }

  blockquote {
    border-left: 3px #DFDFDF solid;
    margin-left: 0;
    padding-left: 15px;
  }

  blockquote p {
    font-family: 'Noto Serif KR', serif;
    font-style: italic;
    font-size: 18px;
  }

  table {
    border-spacing: unset;
    border-collapse: collapse;
    width: 100%;
  }

  th, td {
    padding: 5px 10px 5px 10px;
    border: 1px black solid;
  }

  sup {
    font-size: 11px;
  }
`;

const Preview: React.FC<Props> = ({ content }) => {
  const md = new MarkdownIt({
    html: false,
    xhtmlOut: false,
    breaks: false,
    langPrefix: 'language-',
    linkify: true,
    typographer: true,
    quotes: '“”‘’',
  });

  const convertedContent = md.render(content);

  return <Content dangerouslySetInnerHTML={{ __html: convertedContent }} />
};

export default Preview;
