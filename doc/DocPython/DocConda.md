 $150-$180K
 $65K
# Install

- Install MiniConda: [here](https://docs.conda.io/projects/miniconda/en/latest/index.html)   # base env is created


	conda create -n oblio python=3.11
	conda activate oblio
	conda init zsh --all --set default=oblio

	conda install matplotlib
	conda install scikit-learn
	conda install seaborn
	conda install pyyaml



???
	conda install streamlit



## Commands

	conda list
	conda activate <envname>
	conda deactive
	conda env create --name NEW-NAME
	conda create --name new-env --clone source-env
	conda env list

## Setup Commands

    conda remove tk 
    conda install -c anaconda tk

