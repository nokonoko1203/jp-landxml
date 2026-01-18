# jp-landxml

[WIP]
- GeoTiff変換時に座標が若干ズレるなどしている
- オプションを確認できていない
  - 並列化できているか
  - 出力形式はGeoTIFF以外不要
  - 解像度変更はできるか
- Surfaceの実装を確認できていない
- 法線とか断面とか、GeoTiff変換・地形に関係ない部分を削除

日本の土木分野で標準的に使用されているLandXML（J-LandXML Ver.1.6対応）をパースするRustライブラリです。

## 特徴

- LandXML 1.2 + J-LandXML Ver.1.6拡張対応
- TINデータからDEM（GeoTIFF）生成
- 大容量ファイル（100MB+）対応
- 日本の平面直角座標系（1-19系）自動推定

## インストール

```toml
[dependencies]
jp-landxml = "0.1.0"
```

## 使用方法

### ライブラリとして使用

```rust
use jp_landxml::LandXMLParser;

let parser = LandXMLParser::from_file("sample.xml")?;
let landxml = parser.parse()?;

println!("Version: {}", landxml.version);
println!("Surfaces: {}", landxml.surfaces.len());
```

### CLIツールとして使用

```bash
# CLIツールをビルド
cargo build --features cli --release

# LandXMLファイルの情報を表示
cargo run --features cli -- parse examples/sample_landxml.xml

# JSON形式で出力
cargo run --features cli -- parse examples/sample_landxml.xml -o output.json

# GeoTIFF形式でDEM出力
cargo run --features cli -- export-dem examples/sample_landxml.xml -o output/ --resolution 1.0
```

## GeoTIFF生成の主要オプション

| オプション         | 説明                               | デフォルト |
| ------------------ | ---------------------------------- | ---------- |
| `--resolution`     | DEM解像度（メートル）              | 1.0        |
| `--surface-filter` | サーフェス名フィルタ               | -          |
| `--format`         | 出力形式（geotiff/ascii-grid/xyz） | geotiff    |
| `--all-surfaces`   | 全サーフェスを個別変換             | false      |

## サポートするデータ形式

### 入力
- LandXML 1.2（標準仕様）
- J-LandXML Ver.1.5/1.6（日本拡張仕様）
- エンコーディング: UTF-8

### 出力
- JSON（構造化データ）
- GeoTIFF（ラスタDEM）
- ASCII Grid（ESRI形式）
- XYZ（点群座標）

## 座標系

- 日本の平面直角座標系（1-19系）に対応
- 座標値から適切なEPSGコードを自動推定
- JGD2011基準（EPSG:6669-6687）

## 実装構造

```
src/
├── lib.rs                # ライブラリエントリーポイント
├── parser.rs             # 標準LandXMLパーサー（quick-xml使用）
├── models.rs             # データ構造（Surface, Alignment, Feature等）
├── geometry.rs           # 幾何計算（クロソイド曲線、面積計算等）
├── error.rs              # エラー型定義
├── export.rs             # JSON出力
├── dem/                  # DEM（数値標高モデル）生成モジュール
│   ├── mod.rs            # DEMモジュールエントリーポイント
│   ├── grid.rs           # グリッドDEM処理
│   ├── triangulation.rs  # TIN→DEM変換（標高補間）
│   └── geotiff_writer.rs # GeoTIFF出力（GDAL使用）
└── jlandxml/             # J-LandXML Ver.1.6 拡張対応モジュール
    ├── mod.rs            # J-LandXMLモジュールエントリーポイント
    ├── coordinate_systems.rs # 平面直角座標系1〜19系、測地原子、鉛直原子
    ├── models.rs         # J-LandXML拡張データモデル
    └── parser.rs         # J-LandXML専用パーサー
```

### 主要モジュールの役割

| モジュール | 役割 |
|-----------|------|
| `parser` | XMLストリーミング処理、Surface/Alignment/Feature要素のパース |
| `models` | Point3D, Face, Surface, Alignment等のデータ構造定義 |
| `dem` | TINデータからグリッドDEMへの変換、GeoTIFF出力 |
| `jlandxml` | 日本独自拡張（平面直角座標系、高さ基準変換等） |

## テスト

```bash
# 全テスト実行
cargo test

# DEM生成テスト
cargo test --test dem_tests
```

## 開発要件

- Rust 1.70+
- GDAL（GeoTIFF出力用）

## ライセンス

MIT License
