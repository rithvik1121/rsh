//simple weather app meant to be included in my rsh shell. This is the first real program I am writing on my new laptop with 
//vanilla neovim, no plugins added.
//using the weather.gov api, which currently uses a user-agent string.
//this may change to using an API key instead in the future
//also using openstreetmap API

#include <stdio.h>
#include <curl/curl.h>

int main() {
	printf("Hello World\n");
	



	CURL* curl;
	CURLcode result;

	
	curl = curl_easy_init();
	if (curl == NULL) {
		fprintf(stderr, "HTTP request failed\n");
		return -1;
	}

	curl_easy_setopt(curl, CURLOPT_URL, "https://www.google.com");

	result = curl_easy_perform(curl);
	if(result != CURLE_OK) {
		fprintf(stderr, "Error: %s\n", curl_easy_strerror(result));
		return -1;
	}
	curl_easy_cleanup(curl);
}
