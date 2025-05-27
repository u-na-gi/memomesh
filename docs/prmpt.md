

`app/backend/src/api/auth/login/mod.rs`の実装をいかにしたがってやってください。

以下のルールを必ず守ってください
1. 今回の実装ではhadlerだけの実装とし、全体を実装するわけではありません。

---


いかに実装の方針を示します。

1. リクエストを作る
ユーザー名とパスワードを受け取ります。

ただし、型は、`share/protobuf` 配下に書いてください。
protobufの実装を行う場合、api/auth/loginみたいな階層構造を作って作るようにしてください。生成して別dirにコピーする場合も、
同じような階層構造を作って書くようにしてください。
できるだけシンプルな名前にしてほしい。
rustでprotobufから型を作ります。
コンパイルしたら`app/backend/src/types` 配下に置くようにしてください。

2. リクエストを受け取る
そのまま。

3. ユーザーが存在するか確認
一旦成功した場合のみ書いてください。

4. 入力されたパスワードとハッシュを照合
適当な値を返すだけで良いです。

5. JWTを発行（有効期限5分）
jwtっぽい適当な値を書くだけで良いです

6. セッションIDを生成しサーバーに保存（30日）
適当な値を書くだけで良いです。サーバーに保存は実装しないでいいです。

7. リフレッシュトークンを生成（30日）
適当な値を書くだけで良いです。サーバーに保存は実装しないでいいです。

8. クッキーに session_id, refresh_token を設定（httpOnly, secure, Lax）
これは実装してください。

ただし、以下の実装を参考にしてください

```rust
pub fn create_refresh_token_cookie(
    refresh_token: RefreshToken,
    max_age_seconds: Option<usize>,
) -> Cookie<'static> {
    // defaultは一週間
    let max_age_seconds = match max_age_seconds {
        Some(v) => v,
        None => {
            let sec = get_expiration_epoch(7, crate::components::js_epoch::TimeUnit::Days);
            sec
        }
    };

    // 開発ビルドとリリースビルドで切り替え
    let is_production = cfg!(not(debug_assertions)); // リリースビルド時はtrue、開発ビルド時はfalse

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token.into_val().to_string()))
        .http_only(is_production) // リリースビルドではtrue、開発ビルドではfalse
        .secure(is_production) // リリースビルドではSecureを有効
        .path("/web/token/refresh") // Cookieを送信するパス
        .max_age(Duration::seconds(max_age_seconds as i64));

    refresh_cookie.inner().clone().into_owned()
}
```

corsも必要です。
そちらは app/backend/src/route.rs　に読み込ませられるように書いてください。
app/backend/src/internal/components/cors.rs に実装は書いてください

参考実装は以下のとおりです。

```
use tower_http::cors::{AllowOrigin, CorsLayer};

fn custom_allow_origin(origin: &HeaderValue) -> bool {
    let allowed_substrings = ["localhost", "unagi-id-bwb"];
    let origin_str = origin.to_str().unwrap_or("");
    allowed_substrings
        .iter()
        .any(|&substring| origin_str.contains(substring))
}



    let headers = [AUTHORIZATION, CONTENT_TYPE, preview_authorization];
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin, _| {
            custom_allow_origin(origin)
        }))
        .allow_headers(headers);


// 以下のようにしてrouterにlayerを追加してください
let router = router.layer(cors);
```

ローカル開発(wrangler dev)の実装時には、cookieの実装は（httpOnly, secure, Lax） はhttp only以外ゆるくしてください。

9. JWTはレスポンスのbodyに返す
こちらも1と同じように、protobufで定義を書いてください



UTはまだ書かなくて良いです。
さらにサーバーの実行もしないでいいです。





