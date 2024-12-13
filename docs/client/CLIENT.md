# Client

## General

クライアント部はゲームプログラム本体である。

## Components

汎用性を高めるために、各種UIや物体やをコンポーネントとして`component`モジュール下に定義すること。
また、原則、それを組み合わせて画面を構成すること。

## Map Scene

原則、コンポーネントをイベントで操作する。
例えば、NPCを動かしたい場合、そのNPCの設定を反映した`Actor`コンポーネントのインスタンスを`MapScene.coms.actors`に追加し、そのNPCを動かすイベントを`MapScene.events`に追加する。
`MapScene.events`は`MapScene.coms`の可変参照を使える形で毎フレーム呼び出されるようになっている。
