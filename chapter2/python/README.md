## 各スクリプトファイルの実行
パッケージ管理にpoetryというライブラリを使っているので、インストールしてください。
https://python-poetry.org/docs/
```
poetry install # 依存ライブラリをインストール
poetry run python run_calc_batch_list.py # pure pythonのバッチ計算プログラムを動かす
poetry run python run_calc_batch_numpy.py # numpyのバッチ計算プログラムを動かす
poetry run python run_calc_stream.py # ストリーム計算プログラムを動かす
```
## ディレクトリの解説  
- run_calc_batch_list.py
  - 純粋なpythonのみで移動平均を計算するバッチスクリプトです

- run_calc_batch_numpy.py
  - numpy, pandas, numbaを利用して移動平均を計算するバッチスクリプトです

- run_calc_stream.py
  - stream風に移動平均を計算した時にどうなるかを表現したスクリプトです

- _generate_time_series.py
  - 時系列データを作るスクリプトです(実行不要)

- _plot_time_series.ipynb
  - 時系列データを移動平均とともに描画するノートブックです(実行不要)
  
- pyproject.toml
  - poetryというパッケージ管理ツールのプロジェクトファイルです
  - これを用いて利用する外部ライブラリの管理をしています

- poetry.lock
  - poetryが生成する外部ライブラリのバージョンリストです

## 注意事項
このnumpyはpipを使ってインストールされているのでopenblasが使われます。
anacondaを使うことでより高速化できるかもしれません。

### やって見た結果
anacondaを利用しても大きな差は見られなかった。(むしろanacondaの方が遅い)なぜか？
- 今回の計算にはblasを有効に活用できる行列が絡んだ演算がない。(SIMD最適化ぐらいはされるが、おそらくblasの実装に依らない)
- AMDのCPUなのでintelのCPUほどMKLの恩恵がない？
- anacondaの方が色々とバージョンが古いので、遅くなる。

| Poetry(openblas)                                             | Anaconda(MKL)                                                |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| python version: 3.9.9 (main, Nov 20 2021, 21:30:06) <br/>[GCC 11.1.0]<br/>numpy version: 1.20.3<br/>blas_mkl_info:<br/>  NOT AVAILABLE<br/>blis_info:<br/>  NOT AVAILABLE<br/>openblas_info:<br/>    libraries = ['openblas', 'openblas']<br/>    library_dirs = ['/usr/local/lib']<br/>    language = c<br/>    define_macros = [('HAVE_CBLAS', None)]<br/>blas_opt_info:<br/>    libraries = ['openblas', 'openblas']<br/>    library_dirs = ['/usr/local/lib']<br/>    language = c<br/>    define_macros = [('HAVE_CBLAS', None)]<br/>lapack_mkl_info:<br/>  NOT AVAILABLE<br/>openblas_lapack_info:<br/>    libraries = ['openblas', 'openblas']<br/>    library_dirs = ['/usr/local/lib']<br/>    language = c<br/>    define_macros = [('HAVE_CBLAS', None)]<br/>lapack_opt_info:<br/>    libraries = ['openblas', 'openblas']<br/>    library_dirs = ['/usr/local/lib']<br/>    language = c<br/>    define_macros = [('HAVE_CBLAS', None)]<br/>numba version: 0.54.1<br/>移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f2a41a6d040><br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.531876秒<br/>移動平均計算にかかった時間：0:00:00.027393秒<br/>移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f2a41a6d0d0><br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.523522秒<br/>移動平均計算にかかった時間：0:00:00.117787秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f2a41a6d1f0>)<br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.524294秒<br/>移動平均計算にかかった時間：0:00:00.288164秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f2a41a6d3a0>)<br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.523398秒<br/>移動平均計算にかかった時間：0:00:00.141007秒<br/>移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f2a41a6d040><br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.519580秒<br/>移動平均計算にかかった時間：0:00:05.684404秒<br/>移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f2a41a6d0d0><br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.526259秒<br/>移動平均計算にかかった時間：0:00:00.116782秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f2a41a6d1f0>)<br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.527559秒<br/>移動平均計算にかかった時間：0:00:32.587866秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f2a41a6d3a0>)<br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.521110秒<br/>移動平均計算にかかった時間：0:00:00.048018秒 |python version: 3.8.8 (default, Apr 13 2021, 19:58:26)<br/>[GCC 7.3.0]<br/>numpy version: 1.20.1<br/>blas_mkl_info:<br/>    libraries = ['mkl_rt', 'pthread']<br/>    library_dirs = ['/opt/anaconda/lib']<br/>    define_macros = [('SCIPY_MKL_H', None), ('HAVE_CBLAS', None)]<br/>    include_dirs = ['/opt/anaconda/include']<br/>blas_opt_info:<br/>    libraries = ['mkl_rt', 'pthread']<br/>    library_dirs = ['/opt/anaconda/lib']<br/>    define_macros = [('SCIPY_MKL_H', None), ('HAVE_CBLAS', None)]<br/>    include_dirs = ['/opt/anaconda/include']<br/>lapack_mkl_info:<br/>    libraries = ['mkl_rt', 'pthread']<br/>    library_dirs = ['/opt/anaconda/lib']<br/>    define_macros = [('SCIPY_MKL_H', None), ('HAVE_CBLAS', None)]<br/>    include_dirs = ['/opt/anaconda/include']<br/>lapack_opt_info:<br/>    libraries = ['mkl_rt', 'pthread']<br/>    library_dirs = ['/opt/anaconda/lib']<br/>    define_macros = [('SCIPY_MKL_H', None), ('HAVE_CBLAS', None)]<br/>    include_dirs = ['/opt/anaconda/include']<br/>numba version: 0.53.1<br/><br/><br/>移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f7f338850d0><br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.508564秒<br/>移動平均計算にかかった時間：0:00:00.027632秒<br/>移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f7f33885160><br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.496339秒<br/>移動平均計算にかかった時間：0:00:00.125797秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f7f33885280>)<br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.494668秒<br/>移動平均計算にかかった時間：0:00:00.356287秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f7f33885430>)<br/>移動平均の長さ：7<br/>移動平均の最後の要素：1428573.5714285714<br/>csvロードにかかった時間：0:00:00.497922秒<br/>移動平均計算にかかった時間：0:00:00.189861秒<br/>移動平均計算に使用した関数：<function moving_average_batch_numpy at 0x7f7f338850d0><br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.501834秒<br/>移動平均計算にかかった時間：0:00:18.394249秒<br/>移動平均計算に使用した関数：<function moving_average_batch_pandas at 0x7f7f33885160><br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.537248秒<br/>移動平均計算にかかった時間：0:00:00.131178秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_naive at 0x7f7f33885280>)<br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.524034秒<br/>移動平均計算にかかった時間：0:00:34.673282秒<br/>移動平均計算に使用した関数：CPUDispatcher(<function moving_average_batch_numpy_numba_online at 0x7f7f33885430>)<br/>移動平均の長さ：5000<br/>移動平均の最後の要素：1428216.9284<br/>csvロードにかかった時間：0:00:00.525459秒<br/>移動平均計算にかかった時間：0:00:00.051068秒 |

