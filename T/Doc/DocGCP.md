
- https://console.cloud.google.com.    [Google Cloud Console](spot://Google~Cloud~Console) 

# HowTo

## Start

### Setup Permissions
https://console.cloud.google.com
! AI-Worker-Launcher		// Our project
! I AM & Admin

### Setting Up google-cloud-sdk


// Setup Google Cloud CLI
https://cloud.google.com/sdk/docs/install

	# download for OSX and untar
	# Move to home folder
	mv google-cloud-sdk ~
	cd ~
	
	./google-cloud-sdk/install.sh
	gcloud init
		# this only ran after removing my RJ45 cable


### Run an SV instance


https://console.cloud.google.com
-> 

### Run Docker instance from "base-pipeline-runner"

	# on local machine w/ gcloud CLI
	gcloud compute ssh --ssh-flag="-A" --zone "us-east1-c" --project "ai-worker-launcher" "dan-base-pipeline"

	# on GCP instance
	sudo su ubuntu
	cd ~



# LOG

### 2024-07-18  Working with Juan to create new instances


// Launch Instance from WebConsole

	https://console.cloud.google.com
	! Compute Engine
	! Instance Templates (on left)
	! "..." on base-pipeline-runner -> ! Create VM
		{{edit instance name}}
		! Create
	

// Creating a new GCP image:  (1) Run existing image (2) update 

