ROOT_DIR=docs/

# ローカルサーバで実行
run:
	trunk serve

# docsディレクトリ内にaudioディレクトリがあることを前提とする
public-build:
	trunk build --release --public-url invader-in-browser
	rsync -av dist/ $(ROOT_DIR)