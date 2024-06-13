# peacekeeper_bot

個人的に作成した管理用のBotです  
通報用に設定した任意のカスタム絵文字が一定の数溜まったメッセージを消すことができます    
+ 消す判定は絵文字が追加されたタイミングです  
+ Botが実行される前に通報用のカスタム絵文字が溜まっていた場合には対応していません  

実行する前に以下の環境変数を設定してください  

+ DISCORD_TOKEN → DISCORDBOTのTOKEN  
+ DISCORD_REPORT_EMOJI_ID → 通報用カスタム絵文字のID(Unicode絵文字には対応していません)  
+ DISCORD_USER_ID → 通報用BOTの有効無効を切り替えられるDISCORDユーザをIDで指定  

仕様：  
+ DISCORD_USER_IDで指定したユーザが特定のキーフレーズ(デフォルト:wake_up)を含んだメッセージを投稿するとBotの有効無効が切り替わります  
+ Botが有効な状態で上記で指定されたカスタム絵文字が一定回数(デフォルト10個)以上押されるとBotが投稿に削除する旨の返信を送った後当該の投稿を消します  

![bot_image](./figures/bot_image.png)

Botの導入など参考にしたサイト:
https://zenn.dev/t4t5u0/articles/cd731e0293cf224cb4dc

一応，MIT Licenceです