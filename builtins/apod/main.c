#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>



typedef struct {
    char* string;
    size_t size;
} Response;

typedef struct {
    char* explanation;
    char* title;
    char* url;
    size_t ex_size;
    size_t ti_size;
    size_t url_size;
} MappedResponse;

typedef struct {
    char* bytes;
    size_t height;
    size_t width;
} Image;

size_t write_chunk(void* data, size_t size, size_t nmemb, void* userdata);

int main() {
    char* url;
    FILE* keyptr;
	CURL* curl;
	CURLcode result;
    char** items;

    Response response;
    Response testImage;
    Image image;
    int counter = 0;
    int flag = 0;
    int temp = 0;

    items = malloc(3 * sizeof(char*));


    /*
    keyptr = fopen("api_key", "r");
    if (keyptr == NULL) {
        fprintf(stderr, "Failed to retrieve API Key\n");
        return -1;
    }

    fread(api_key, sizeof(char), 41, keyptr);
    printf("%sEND\n", api_key);
    api_key[41] = '\0';
    */
	
	curl = curl_easy_init();
	if (curl == NULL) {
		fprintf(stderr, "HTTP request failed\n");
		return -1;
	}

    //initial api call
    response.string = malloc(1);
    response.size = 0;
	curl_easy_setopt(curl, CURLOPT_URL, "https://api.nasa.gov/planetary/apod?api_key=KfGpfrX7aoKc7Od75CmESNiZWHsYvq05QHXkYPwB");
    //curl_easy_setopt(curl, CURLOPT_USERAGENT, "picture-of-the-day");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_chunk);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void*) &response);
	result = curl_easy_perform(curl);

    

//    printf("%s\n", response.string);

	if(result != CURLE_OK) {
		fprintf(stderr, "Error: %s\n", curl_easy_strerror(result));
		return -1;
	}

    char* blob = strtok(response.string, "\"");

    while(blob) {
        //printf("%d: %s\n",counter, blob);

        if (strcmp(blob, "url")==0) {
            flag = 1;
            temp = counter;
        }


        if (strcmp(blob, "title")==0) {
            flag = 2;
            temp=counter;
        }

        if (strcmp(blob, "explanation")==0) {
            flag = 3;
            temp=counter;
        }

        if (counter == (temp+2)) {
            if (flag != 0) {
                items[flag-1]=malloc(strlen(blob) * sizeof(char));
                strcpy(items[flag-1], blob);
                flag=0;
            }
        }

        counter++;
        blob = strtok(NULL, "\"");
    }


    /*
    for(int i = 0; i < 3; i++) {
        printf("%s\n", items[i]);
    }
    */


    //retrieve image from url returned in API call
    image.bytes = malloc(1);
    image.height = 0;
    image.width = 0;

    testImage.string= malloc(1);
    testImage.size = 0;
    printf("%s\n", items[0]);


	curl_easy_setopt(curl, CURLOPT_URL, items[0]);
    //curl_easy_setopt(curl, CURLOPT_USERAGENT, "picture-of-the-day");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_chunk);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void*) &testImage);
    //printf("%s\n", testImage.string);

	result = curl_easy_perform(curl);


	if(result != CURLE_OK) {
		fprintf(stderr, "Error: %s\n", curl_easy_strerror(result));
		return -1;
	}

	curl_easy_cleanup(curl);

    free(response.string);
}


size_t write_chunk( void* data, size_t size, size_t nmemb, void* userdata) {
    size_t real_size = size * nmemb;

    Response* response = (Response*) userdata;

    char* ptr = realloc(response->string, response->size + real_size + 1);

    if(ptr==NULL) {
        return CURL_WRITEFUNC_ERROR;
    }

    response->string = ptr;
    memcpy(&(response->string[response->size]), data, real_size);

    response->size += real_size;
    response->string[response->size]='\0';
    
    return real_size;
}
