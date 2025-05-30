# LandXML/J-LandXML パース用 Rust ライブラリ設計計画書

## 🚧 現在の実装状況（2025年5月30日時点）

### ✅ 実装完了
- **基本XMLパース基盤**: quick-xmlによる効率的なXMLパース
- **Surfaceデータ処理**: TINモデル（点群・三角形面）の完全パース・処理
- **座標系処理**: CoordinateSystemの基本パース（標準LandXMLのみ、J-LandXML拡張未対応）
- **データ構造定義**: 主要な構造体（LandXML、Surface、Alignment等）
- **幾何計算**: クロソイド曲線計算、面積計算
- **JSON出力**: パースしたデータの構造化JSON出力
- **エラーハンドリング**: 包括的なエラー処理体系
- **テスト環境**: 基本テスト・大容量ファイルテスト（69MB実データ対応）
- **性能**: ストリーミング処理による大容量ファイル対応

### ⚠️ 部分実装
- **Alignment要素**: 基本構造定義済み、詳細パース未実装
- **Feature要素**: 基本構造定義済み、Property動的プロパティ未実装
- **CrossSections**: データ構造定義済み、詳細パース未実装

### ❌ 未実装（優先度高）
- **J-LandXML座標系拡張仕様**: horizontalCoordinateSystemName対応、平面直角座標系1系～19系
- **J-LandXML Ver.1.6拡張機能**: WidthStakePnts、非表示属性、開放/閉合区分
- **XSDスキーマ検証**: スキーマファイルによる厳密な検証
- **Property要素処理**: Feature内の動的プロパティ管理
- **Profile・Superelevation**: 縦断線形・横断勾配処理
- **エンコーディング対応**: Shift_JIS等多言語対応
- **CLI機能**: コマンドラインツール

**進捗率**: 約30%（基盤部分は堅牢、拡張機能実装が必要）

## 🎯 最新タスク：J-LandXML座標系拡張仕様対応（2025年5月30日追加）

### 背景・要件
- `Coordinatesystem`の`horizontalCoordinateSystemName="1(X,Y)"`属性で平面直角座標系1系を指定
- J-LandXML拡張仕様として標準LandXMLと分離した実装が必要
- 日本の平面直角座標系1系～19系の完全対応

### 実装計画

#### **フェーズ1**: 基盤構築 🏗️
- [ ] `src/jlandxml/` モジュール作成
- [ ] 平面直角座標系定義（1系～19系、EPSGコード対応）
- [ ] J-LandXML用拡張CoordinateSystemモデル定義
- [ ] 座標系名パース機能（正規表現による`"N(X,Y)"`形式解析）

#### **フェーズ2**: パーサー拡張 ⚙️
- [ ] `horizontalCoordinateSystemName`属性パース機能
- [ ] J-LandXML専用パーサー実装（標準LandXMLパーサーを継承）
- [ ] 座標系名 → 平面直角座標系マッピング機能
- [ ] エラーハンドリング（不正な座標系名、対応外の系番号）

#### **フェーズ3**: 統合・テスト 🧪
- [ ] 既存APIとの統合（後方互換性保持）
- [ ] J-LandXMLサンプルファイル作成・テスト
- [ ] 座標変換機能（オプション）：平面直角座標系間、地理座標系との変換
- [ ] ドキュメント更新・使用例追加

### 技術仕様

#### 新モジュール構造
```
src/
├── jlandxml/                    # 新規作成
│   ├── mod.rs                   # モジュールエントリーポイント
│   ├── models.rs                # J-LandXML拡張モデル
│   ├── parser.rs                # J-LandXML専用パーサー
│   └── coordinate_systems.rs    # 平面直角座標系定義・変換
```

#### 平面直角座標系対応
```rust
pub enum JapanPlaneCoordinateSystem {
    Zone1,   // EPSG:6669 - 長崎、鹿児島県の一部
    Zone2,   // EPSG:6670 - 福岡、佐賀、熊本、大分、宮崎県
    // ... Zone19まで（EPSG:6687）
}

pub struct JLandXmlCoordinateSystem {
    pub base: CoordinateSystem,                             // 標準LandXML部分
    pub horizontal_coordinate_system_name: Option<String>,  // J-LandXML拡張
    pub plane_coordinate_zone: Option<JapanPlaneCoordinateSystem>,
}
```

#### パース対象属性
- `horizontalCoordinateSystemName="1(X,Y)"` → 平面直角座標系1系
- `horizontalCoordinateSystemName="9(X,Y)"` → 平面直角座標系9系
- その他の系（2～19系）も同様にサポート

### 優先度・期限
- **優先度**: 高（J-LandXML対応の基盤機能）
- **想定期間**: 1-2週間
- **依存関係**: 既存の標準LandXMLパーサーに依存、破壊的変更なし

## 📍 CoordinateSystem仕様詳細（完全版）

### 1. J-LandXML CoordinateSystem要素の完全仕様

#### 1.1 要素構造と役割

```xml
<CoordinateSystem
    name="CRS1"
    horizontalDatum="JGD2000"
    verticalDatum="O.P"
    horizontalCoordinateSystemName="9(X,Y)"
    desc="第９系">
  <Feature>
    <Property label="differTP" value="-1.3000"/>
  </Feature>
</CoordinateSystem>
```

| パス                              | 型 / 書式 | 必須 | 説明                                                                  |
| --------------------------------- | --------- | ---- | --------------------------------------------------------------------- |
| `CoordinateSystem`                | ―         | ○    | 空間参照系のまとまりを表すルート要素                                  |
| `@name`                           | xs:string | ○    | CRS 識別子（自由記述）                                                |
| `@horizontalDatum`                | xs:string | ○    | **測地原子**（表１の列挙値）                                          |
| `@verticalDatum`                  | xs:string | ○    | **鉛直原子**（表２の列挙値）                                          |
| `@horizontalCoordinateSystemName` | xs:string | ○    | **水平座標系の基準名**（表３の列挙値）                                |
| `@desc`                           | xs:string | △    | 補足的な説明や系番号など自由記述                                      |
| `Feature`                         | ―         | △    | 属性補足用の子要素（複数可）                                          |
| `Property@label`                  | xs:string | ○    | `Feature` 内のキー名                                                  |
| `Property@value`                  | xs:string | ○    | `Feature` 内の値。ここでは **differTP** が T.P との差（単位 m）を保持 |

> **実装ポイント**
>
> * `verticalDatum` が **T.P** 以外の場合は **differTP** プロパティを付与し、T.P との差を必ず明示する。
> * 単位はメートル、符号は「基準面 – T.P」。例：O.P は T.P より 1.300 m 低いので `-1.3000`。

### 2. 属性値の詳細定義

#### 表１　水平位置の測地原子（`horizontalDatum`）

| 値          | 日本語名称     | 備考                               |
| ----------- | -------------- | ---------------------------------- |
| **JGD2000** | 日本測地系2000 | 最新の全国測地原子（GRS80 楕円体） |
| **JGD2011** | 日本測地系2011 | 東日本大震災後の再測量に対応       |
| **TD**      | 日本測地系     | 旧 Tokyo Datum（Bessel 楕円体）    |

#### 表２　鉛直原子（`verticalDatum`）

| 値        | 対象河川・水域              | T.P との差 (m)    | 備考                                             |
| --------- | --------------------------- | ----------------- | ------------------------------------------------ |
| **T.P**   | 東京湾中等潮位              | 0 (基準)          | Tokyo Peil                                       |
| **K.P**   | 北上川                      | -0.8745           | Kitakami Peil                                    |
| **S.P**   | 鳴瀬川                      | -0.0873           | Same Peil                                        |
| **Y.P**   | 利根川                      | -0.8402           | Tone Peil                                        |
| **A.P**   | 荒川・中川・多摩川 / 吉野川 | -1.1344 / -0.8333 | Arakawa Peil（関東）と同名だが差が異なるため注意 |
| **O.P**   | **淀川**                    | **-1.3000**       | Osaka Peil                                       |
| **T.P.W** | 渡川                        | +0.113            | Tosa Peil Watarigawa                             |
| **B.S.L** | 琵琶湖                      | +84.371           | Biwa Surface Level                               |

#### 表３　水平座標系の基準名（`horizontalCoordinateSystemName`）

| 値                | 説明                          | 適用地域（概略）                                                                                  |
| ----------------- | ----------------------------- | ------------------------------------------------------------------------------------------------- |
| 1(X,Y) 〜 19(X,Y) | 平面直角座標系 第 I ～ XIX 系 | 国土地理院定義の 19 系。<br/>値 = 系番号 + "(X,Y)"。例：**9(X,Y)** = 第 IX 系（関西・近畿中心）。 |
| 2系               | 6670                          | 福岡、佐賀、熊本、大分、宮崎県                                                                    | 131°00′E | 33°00′N |
| 3系               | 6671                          | 山口、島根、広島県                                                                                | 132°10′E | 34°20′N |
| 4系               | 6672                          | 香川、愛媛、徳島、高知県                                                                          | 133°30′E | 33°00′N |
| 5系               | 6673                          | 兵庫、鳥取、岡山県                                                                                | 134°20′E | 34°40′N |
| 6系               | 6674                          | 京都、大阪、福井、滋賀、三重、奈良、和歌山県                                                      | 136°00′E | 36°00′N |
| 7系               | 6675                          | 石川、富山、岐阜、愛知県                                                                          | 137°10′E | 36°00′N |
| 8系               | 6676                          | 新潟、長野、山梨、静岡県                                                                          | 138°30′E | 36°00′N |
| 9系               | 6677                          | 東京、福島、栃木、茨城、埼玉、千葉、群馬、神奈川県                                                | 139°50′E | 36°00′N |
| 10系              | 6678                          | 青森、秋田、山形、岩手、宮城県                                                                    | 140°50′E | 40°00′N |
| 11系              | 6679                          | 小笠原諸島                                                                                        | 142°15′E | 26°00′N |
| 12系              | 6680                          | 北海道西部                                                                                        | 142°15′E | 44°00′N |
| 13系              | 6681                          | 北海道中央部                                                                                      | 144°15′E | 44°00′N |
| 14系              | 6682                          | 北海道東部                                                                                        | 142°15′E | 44°00′N |
| 15系              | 6683                          | 沖縄県本島                                                                                        | 127°30′E | 26°00′N |
| 16系              | 6684                          | 沖縄県西表島                                                                                      | 124°00′E | 26°00′N |
| 17系              | 6685                          | 沖縄県硫黄鳥島                                                                                    | 131°00′E | 26°00′N |
| 18系              | 6686                          | 沖縄県小笠原硫黄島                                                                                | 136°00′E | 20°00′N |
| 19系              | 6687                          | 沖縄県南鳥島                                                                                      | 153°59′E | 26°00′N |

#### 3. 設計上の留意点

1. **正規化チェック**
   * `horizontalDatum` と `horizontalCoordinateSystemName` の組合せは論理的に一致させる。例：第 IX 系なら JGD2000 または JGD2011 が一般的。

2. **高さ系変換**
   * アプリ側で標高を T.P 系に統一する場合は `differTP` を利用して
     `標高_TP = 観測標高 + differTP` の補正を行う。

3. **互換性**
   * 旧 Tokyo Datum (TD) ＋ 第 I～XIX 系というレガシー組合せも許容することで既存データ移行を容易にする。

4. **バリデーション**
   * 可能であれば XML スキーマ (XSD) 側で列挙型を定義し、上記の表１〜表３に無い値を拒否する。

5. **国際化**
   * `@name` や `@desc` は多言語での UI 表示を想定し UTF-8 固定、長さ制限は 255 byte 程度を推奨。

### 4. 具体例：淀川・第 IX 系の場合

```xml
<CoordinateSystem
    name="Yodo-CRS"
    horizontalDatum="JGD2000"
    verticalDatum="O.P"
    horizontalCoordinateSystemName="9(X,Y)"
    desc="淀川・平面直角座標系第IX系">
  <Feature>
    <!-- O.P は T.P より 1.3 m 低い -->
    <Property label="differTP" value="-1.3000"/>
  </Feature>
</CoordinateSystem>
```

* `verticalDatum="O.P"` → **淀川** 高程基準
* `differTP = -1.3000` → T.P より 1.300 m 低い
* `horizontalCoordinateSystemName="9(X,Y)"` → 関西一帯で用いる第 IX 系平面直角座標

### 5. Rust実装での構造体定義

#### 5.1 拡張CoordinateSystemモデル
```rust
// src/jlandxml/models.rs - 新規実装予定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JLandXmlCoordinateSystem {
    // 基本属性
    pub name: String,
    pub desc: Option<String>,

    // 測地・座標系定義
    pub horizontal_datum: HorizontalDatum,
    pub vertical_datum: VerticalDatum,
    pub horizontal_coordinate_system_name: String,

    // 高さ系補正
    pub differ_tp: Option<f64>,  // T.Pとの差（メートル）

    // 標準LandXML互換性
    pub epsg_code: Option<String>,
    pub proj4_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HorizontalDatum {
    JGD2000,  // 日本測地系2000
    JGD2011,  // 日本測地系2011
    TD,       // Tokyo Datum（旧日本測地系）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerticalDatum {
    TP,       // 東京湾平均海面（基準）
    KP,       // 北上川基準点
    SP,       // 鳴瀬川基準点
    YP,       // 利根川基準点
    AP,       // 荒川基準点
    OP,       // 淀川基準点（大阪）
    TPW,      // 渡川基準点
    BSL,      // 琵琶湖水準面
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlaneRectangularSystem {
    Zone1,  Zone2,  Zone3,  Zone4,  Zone5,
    Zone6,  Zone7,  Zone8,  Zone9,  Zone10,
    Zone11, Zone12, Zone13, Zone14, Zone15,
    Zone16, Zone17, Zone18, Zone19,
}
```

#### 5.2 パース処理の実装
```rust
impl JLandXmlCoordinateSystem {
    /// horizontalCoordinateSystemNameから系番号を抽出
    pub fn parse_zone(coord_name: &str) -> Result<PlaneRectangularSystem, CoordinateSystemError> {
        let re = regex::Regex::new(r"^(\d{1,2})\(X,Y\)$").unwrap();
        if let Some(captures) = re.captures(coord_name) {
            let zone_num: u8 = captures[1].parse()
                .map_err(|_| CoordinateSystemError::InvalidZoneNumber)?;

            match zone_num {
                1 => Ok(PlaneRectangularSystem::Zone1),
                2 => Ok(PlaneRectangularSystem::Zone2),
                // ... 3-18
                19 => Ok(PlaneRectangularSystem::Zone19),
                _ => Err(CoordinateSystemError::InvalidZoneNumber),
            }
        } else {
            Err(CoordinateSystemError::InvalidFormat(coord_name.to_string()))
        }
    }

    /// T.P基準への標高変換
    pub fn to_tp_elevation(&self, raw_elevation: f64) -> f64 {
        match self.differ_tp {
            Some(diff) => raw_elevation + diff,
            None => raw_elevation, // T.P基準またはdifferTP未設定
        }
    }

    /// 指定高さ系からT.P基準への変換
    pub fn get_tp_offset(&self) -> f64 {
        match self.vertical_datum {
            VerticalDatum::TP => 0.0,
            VerticalDatum::KP => -0.8745,
            VerticalDatum::SP => -0.0873,
            VerticalDatum::YP => -0.8402,
            VerticalDatum::AP => -1.1344, // 関東のA.P
            VerticalDatum::OP => -1.3000,
            VerticalDatum::TPW => 0.113,
            VerticalDatum::BSL => 84.371,
        }
    }
}
```

### 6. 実装チェックリスト ✔️

* [ ] 属性値が列挙表（表１〜表３）に含まれているか
* [ ] `verticalDatum ≠ "T.P"` の場合に必ず `differTP` を設定したか
* [ ] 系番号と測地原子の整合性を確認したか
* [ ] XSD で列挙制約と型チェックを定義したか
* [ ] ドキュメントに値の意味と補正式を注記したか
* [ ] 正規表現による`horizontalCoordinateSystemName`の検証を実装したか
* [ ] T.P基準への標高変換処理を実装したか
* [ ] 旧測地系（TD）との互換性を考慮したか
* [ ] エラーハンドリングで適切な例外を定義したか
* [ ] 単体テストで各列挙値のパースを検証したか

---

このライブラリは、日本の土木分野におけるDX推進と生産性向上に貢献することを目指しています。オープンソースとして公開し、業界全体でのデータ連携基盤の充実を図ります。
