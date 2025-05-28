# LandXML/J-LandXML パース用 Rust ライブラリ設計計画書

## プロジェクト概要

本プロジェクトは、日本の土木分野で標準的に使用されているLandXML（特にJ-LandXML Ver.1.6）を効率的にパースするRustライブラリの開発を目的とします。

### 背景
- LandXMLは土木分野の設計・測量データ交換のオープンXML形式
- J-LandXMLは日本の国土交通省が策定した拡張仕様（Ver.1.6が最新）
- 測量→設計→施工→維持管理の全工程でデータ連携の中核を担う
- 国交省直轄事業でのJ-LandXML提出が義務化

## 要件定義

### 1. 機能要件

#### 1.1 基本パース機能
- [x] LandXML 1.2 標準仕様の読み取り
- [x] J-LandXML Ver.1.6 拡張仕様の対応
- [x] XMLスキーマ検証
- [x] エラーハンドリング（不正なXML、仕様違反）

#### 1.2 主要データ要素の対応
- [x] **Surface要素**：TINモデル、地形サーフェス（ExistingGround等）
- [x] **Alignments要素**：道路平面線形、クロソイド曲線
- [x] **Profile要素**：縦断線形、勾配情報
- [x] **CrossSections要素**：横断面、路肩幅、片勾配
- [x] **Feature要素**：J-LandXML拡張項目（累加距離標、幅杭座標等）
- [x] **CoordinateSystem要素**：座標系情報

#### 1.3 J-LandXML固有機能
- [x] 日本固有の線形要素（クロソイドパラメータ）
- [x] 特殊な片勾配すりつけパターン
- [x] 累加距離標（測点）情報
- [x] 幅杭座標（Ver.1.6で追加）
- [x] Feature要素による拡張属性

#### 1.4 出力機能
- [x] パースしたデータの構造化表現
- [x] JSON形式でのエクスポート
- [x] デバッグ用の情報出力

### 2. 非機能要件

#### 2.1 性能要件
- 大容量ファイル（数百MB）の効率的処理
- メモリ使用量の最適化
- ストリーミング処理対応

#### 2.2 保守性要件
- モジュラー設計による拡張性
- 包括的なテストカバレッジ
- 詳細なドキュメント

#### 2.3 互換性要件
- LandXML 1.2 完全対応
- J-LandXML Ver.1.5/1.6 対応
- 将来バージョンへの拡張性

## アーキテクチャ設計

### 1. モジュール構成

```
jp-landxml/
├── src/
│   ├── lib.rs              # ライブラリエントリポイント
│   ├── parser/             # XMLパース機能
│   │   ├── mod.rs
│   │   ├── landxml.rs      # LandXML基本パーサー
│   │   ├── j_landxml.rs    # J-LandXML拡張パーサー
│   │   └── validation.rs   # スキーマ検証
│   ├── models/             # データ構造定義
│   │   ├── mod.rs
│   │   ├── core.rs         # 基本構造体
│   │   ├── surface.rs      # Surface関連
│   │   ├── alignment.rs    # Alignment関連
│   │   ├── profile.rs      # Profile関連
│   │   ├── cross_section.rs # CrossSection関連
│   │   └── feature.rs      # Feature拡張
│   ├── geometry/           # 幾何計算
│   │   ├── mod.rs
│   │   ├── coordinate.rs   # 座標変換
│   │   ├── clothoid.rs     # クロソイド曲線
│   │   └── tin.rs          # TIN処理
│   ├── export/             # 出力機能
│   │   ├── mod.rs
│   │   ├── json.rs         # JSON出力
│   │   └── debug.rs        # デバッグ出力
│   ├── error.rs            # エラー定義
│   └── utils.rs            # ユーティリティ
├── tests/                  # テストファイル
├── examples/               # 使用例
├── docs/                   # ドキュメント
└── schemas/                # XMLスキーマ定義
```

### 2. 主要データ構造

```rust
// 基本構造
pub struct LandXML {
    pub version: String,
    pub coordinate_system: Option<CoordinateSystem>,
    pub surfaces: Vec<Surface>,
    pub alignments: Vec<Alignment>,
    pub features: Vec<Feature>, // J-LandXML拡張
}

// 地形サーフェス
pub struct Surface {
    pub name: String,
    pub surface_type: SurfaceType, // ExistingGround, DesignGround, etc.
    pub definition: SurfaceDefinition,
}

// 線形
pub struct Alignment {
    pub name: String,
    pub coord_geom: CoordGeom,
    pub profile: Option<Profile>,
    pub cross_sections: Vec<CrossSection>,
}

// J-LandXML拡張
pub struct Feature {
    pub code: String,
    pub properties: HashMap<String, String>,
    pub geometry: Option<FeatureGeometry>,
}
```

## タスク分割・実装計画

### フェーズ1: 基盤整備（週1-2）

#### タスク1.1: プロジェクト初期設定
- [x] Cargo.toml依存関係設定
- [x] 基本モジュール構造作成
- [x] エラーハンドリング設計
- [x] テスト環境構築

#### タスク1.2: XMLパース基盤
- [x] XMLパーサーライブラリ選定（quick-xml or roxmltree）
- [x] 基本XMLパース機能実装
- [x] スキーマ検証機能実装

#### タスク1.3: 基本データ構造
- [x] 基本構造体定義
- [x] 座標系・測地系対応
- [x] デバッグ出力機能

### フェーズ2: 基本要素実装（週3-4）

#### タスク2.1: Surface要素
- [x] TINモデルパース機能
- [x] 地形サーフェス処理
- [x] 三角網データ構造

#### タスク2.2: Alignment要素
- [x] 平面線形パース
- [x] 直線・円弧・クロソイド対応
- [x] 線形計算機能

#### タスク2.3: Profile/CrossSection要素
- [x] 縦断線形処理
- [x] 横断面データ処理
- [x] 勾配・幅員計算

### フェーズ3: J-LandXML拡張対応（週5-6）

#### タスク3.1: Feature要素実装
- [x] Feature要素パース機能
- [x] 拡張属性処理
- [x] 動的プロパティ管理

#### タスク3.2: 日本固有要素
- [x] クロソイドパラメータ拡張
- [x] 累加距離標処理
- [x] 幅杭座標（Ver.1.6）対応
- [x] 片勾配すりつけパターン

#### タスク3.3: バージョン対応
- [x] Ver.1.5/1.6差分対応
- [x] 下位互換性確保

### フェーズ4: 品質向上・最適化（週7-8）

#### タスク4.1: テスト強化
- [x] 単体テスト整備
- [x] 統合テスト作成
- [x] 実データテスト

#### タスク4.2: 性能最適化
- [x] メモリ使用量最適化
- [x] 大容量ファイル対応
- [x] ストリーミング処理

#### タスク4.3: エラーハンドリング
- [x] 詳細エラーメッセージ
- [x] 回復可能エラー処理
- [x] ログ機能

### フェーズ5: 出力・統合機能（週9-10）

#### タスク5.1: 出力機能
- [x] JSON形式エクスポート
- [x] CSV形式エクスポート
- [x] デバッグ情報出力

#### タスク5.2: CLI・サンプル
- [x] コマンドラインツール
- [x] 使用例作成
- [x] サンプルデータ整備

#### タスク5.3: ドキュメント
- [x] APIドキュメント
- [x] 使用方法ガイド
- [x] J-LandXML対応表

## 技術選定

### 依存ライブラリ

```toml
[dependencies]
# XMLパース
quick-xml = "0.30"           # 高速XMLパーサー
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"          # JSON出力用

# 幾何計算
nalgebra = "0.32"           # 線形代数
geo = "0.26"                # 地理空間計算

# エラーハンドリング
thiserror = "1.0"           # エラー定義
anyhow = "1.0"              # エラー伝播

# 文字列処理
regex = "1.0"               # 正規表現
encoding_rs = "0.8"         # 文字エンコーディング

[dev-dependencies]
tokio-test = "0.4"          # 非同期テスト
tempfile = "3.0"            # テスト用一時ファイル
```

### 開発環境
- Rust 1.70+ (edition 2021)
- cargo-doc (ドキュメント生成)
- cargo-test (テスト実行)
- cargo-clippy (静的解析)

## 検証・テスト戦略

### 1. テストデータ
- 国交省サンプルデータ
- OCF認証用テストデータ
- 各種ソフトウェア出力データ
- 異常ケース・境界値データ

### 2. テスト観点
- 標準LandXML 1.2互換性
- J-LandXML Ver.1.5/1.6対応
- 大容量ファイル処理
- 文字エンコーディング処理
- エラー処理

### 3. 品質指標
- テストカバレッジ >90%
- メモリ使用量 <100MB（通常ファイル）
- パース速度 >1MB/s

## リスク・課題

### 技術的リスク
1. **仕様の複雑性**: J-LandXML拡張仕様の完全理解
2. **性能問題**: 大容量XMLファイルの処理
3. **互換性**: 各ソフトウェアの出力差異

### 対応策
1. 段階的実装とテスト強化
2. ストリーミング処理とメモリ最適化
3. 実データテストと検証

## 成果物

### プライマリ成果物
- jp-landxml Rustライブラリ
- コマンドラインツール
- APIドキュメント

### セカンダリ成果物
- 使用例・サンプルコード
- J-LandXML対応表
- 性能ベンチマーク

## 今後の拡張予定

### 短期（3ヶ月）
- WebAssembly対応
- Python/Node.jsバインディング
- オンラインビューワ

### 中期（6ヶ月）
- IFC-Infra連携機能
- 3Dビジュアライゼーション
- CADソフト連携API

### 長期（1年）
- リアルタイムデータ処理
- クラウド連携機能
- AI活用データ解析

---

このライブラリは、日本の土木分野におけるDX推進と生産性向上に貢献することを目指しています。オープンソースとして公開し、業界全体でのデータ連携基盤の充実を図ります。
