import React, { useState } from 'react';
import styles from './Ruitter.module.scss';

const API_SIGN_UP_PATH = '/api/users';
const API_LOG_IN_PATH = '/api/sessions';
const API_TWEET_PATH = '/api/user_tweets';
const API_FOLLOW_PATH = '/api/follow_relations';
const API_TIMELINE_PATH = '/api/pages/timeline';

type TimelineItem = {
  name: string,
  content: string,
};

const createPostParam = ({obj}: {obj: Record<string, unknown>}) => ({
  method: "POST",
  headers: {"Content-Type": "application/json; charset=utf-8"},
  body: JSON.stringify(obj),
});

const Ruitter: ()=>JSX.Element =() => {
  const [signUpName, setSignUpName] = useState<string>('');
  const [logInName, setLogInName] = useState<string>('');
  const [tweetDraft, setTweetDraft] = useState<string>('');
  const [followeeName, setFolloweeName] = useState<string>('');
  const [serverTexts, setServerTexts] = useState<string[]>([]);
  const [tweets, setTweets] = useState<TimelineItem[]>([]);

  const onSignUp = async () => {
    const res = await fetch(API_SIGN_UP_PATH, createPostParam({obj: {name: signUpName}})); 
    setServerTexts([res.ok ? `ユーザー登録成功: ${logInName}` : 'ユーザー登録失敗']);
  }

  const onLogin = async () => {
    const res = await fetch(API_LOG_IN_PATH, createPostParam({obj: {name: logInName}})); 
    setServerTexts([res.ok ? `ログイン成功` : 'ログイン失敗']);
  }

  const onFollow = async () => {
    const res = await fetch(API_FOLLOW_PATH, createPostParam({obj: {name: followeeName}})); 
    setServerTexts([res.ok ? `フォロー成功` : 'フォロー失敗']);
  }

  const onTweet = async () => {
    const res = await fetch(API_TWEET_PATH, createPostParam({obj: {content: tweetDraft}})); 
    setServerTexts([res.ok ? `ツイート成功` : 'ツイート失敗']);
  }

  const onFetchTimeLine = async () => {
    const res = await fetch(API_TIMELINE_PATH); 
    if(res.ok) {
      setTweets(await res.json());
    }
    setServerTexts([res.ok ? `タイムライン取得成功` : 'タイムライン取得失敗']);
  }

  return (
      <div>
        <div>
          {serverTexts.map((text, index)=><p key={`${text}_${index}`}>{text}</p>)}
        </div>
        <div className={styles.panes}>
        <div >
          <h2>
            ユーザ管理
          </h2>
          <input
              style={{ width: '100%' }}
              value={signUpName}
              onChange={event => { setSignUpName(event.target.value); }}
              placeholder='your new name'
          />
          <button type="button" onClick={onSignUp}>ユーザー登録</button>

          <input
              style={{ width: '100%' }}
              value={logInName}
              onChange={event => { setLogInName(event.target.value); }}
              placeholder='your registered name'
          />
          <button type="button" onClick={onLogin}>ログイン</button>

          <input
              style={{ width: '100%' }}
              value={followeeName}
              onChange={event => { setFolloweeName(event.target.value); }}
              placeholder='name to follow'
          />
          <button type="button" onClick={onFollow}>フォローする</button>
        </div>
        <div>
          <h2>
            メモを投稿する
          </h2>
          <textarea
              rows={8}
              style={{ width: '100%' }}
              value={tweetDraft}
              onChange={event => { setTweetDraft(event.target.value); }}
              placeholder='今考えていることを書いてみる'
          />
          <button type="button" onClick={onTweet}>ツイート</button>
        </div>
        <div>
          <h2>
            タイムライン
          </h2>
          <button type="button" onClick={onFetchTimeLine}>タイムライン取得</button>
          {
            tweets.map((tweet, index)=><div key={`${index}_${tweet.name}_${tweet.content}`}>{`${tweet.name}: ${tweet.content}`}</div>)
          }
            </div>
        </div>
      </div>
  );
}

export { Ruitter };
