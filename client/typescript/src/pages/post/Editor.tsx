import React, { ChangeEventHandler, FocusEventHandler, useEffect, useState } from 'react';
import styled from 'styled-components';
import TextareaAutosize from 'react-textarea-autosize';

interface Props {
  content: string;
  onBlur: FocusEventHandler<any>;
  onChange: ChangeEventHandler<any>;
}

const TextArea = styled(TextareaAutosize)`
  max-width: 100%;
  min-height: 500px;
  margin-top: 15px;
  padding: 5px;
  font-family: sans-serif;
  font-size: 16px;
  border: 0;
  resize: none;
  line-height: 180%;
`;

const Editor: React.FC<Props> = ({ content, onBlur, onChange}) => {
  return <TextArea
      placeholder='Content'
      value={content}
      onBlur={onBlur}
      onChange={onChange}
    />
};

export default Editor;
