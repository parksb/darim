import React, { ChangeEventHandler, FocusEventHandler } from 'react';
import styled from 'styled-components';
import TextareaAutosize from 'react-textarea-autosize';

import I18n from "../../utils/i18n";

interface Props {
  content: string;
  onBlur: FocusEventHandler<any>;
  onChange: ChangeEventHandler<any>;
}

const TextArea = styled(TextareaAutosize)`
  max-width: 100%;
  min-height: 500px;
  padding: 5px;
  font-family: sans-serif;
  font-size: 16px;
  border: 0;
  resize: none;
  line-height: 180%;
`;

const Editor: React.FC<Props> = ({ content, onBlur, onChange}) => {
  const i18n = new I18n({
    content: {
      'ko-KR': '내용',
      'en-US': 'Content',
    },
  });

  return <TextArea
      placeholder={i18n.text('content')}
      value={content}
      onBlur={onBlur}
      onChange={onChange}
    />
};

export default Editor;
