ROOT_DIR=docs/

# ローカルサーバで実行
run:
	trunk serve

# docsディレクトリ内にaudioディレクトリが用意されていることを前提とする
public-build:
	rm $(ROOT_DIR)invader-in-browser-* $(ROOT_DIR)layout-*
	trunk build --release --public-url invader-in-browser
	rsync -av dist/ $(ROOT_DIR)
