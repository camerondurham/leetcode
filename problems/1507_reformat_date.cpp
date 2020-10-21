#include<sstream>
#include<string>
#include<unordered_map>
using namespace std;
class Solution {
public:
    inline static unordered_map<string, string> months = {
       {"Jan","01"}, {"Feb","02"}, {"Mar","03"}, {"Apr","04"}, {"May","05"}, {"Jun","06"}, {"Jul","07"}, {"Aug","08"}, {"Sep","09"}, {"Oct","10"}, {"Nov","11"}, {"Dec","12"}
    };
    bool notWhitespace(char c){
        return c != ' ';
    }

    string reformatDate(string date) {
        string day("");
        string month("");
        string year("");
        for(int i = 0; i < date.size(); i++){
            if(i < 2 && isdigit(date[i])){
                day += date[i];
            }
            else if(i > 3 && notWhitespace(date[i]) && isdigit(date[i]) == false){
                for(int j = 0; j < 3; j++){
                    if(notWhitespace(date[i+j]))
                        month += date[i+j];
                }
                i += 2;

            } else if(i > 6 && notWhitespace(date[i])) {
                for(int j = 0; j < 4 && i+j < date.size(); j++){
                    year += date[i+j];
                }
                i += 3;
            }
        }
        if(day.size() < 2)
            day = '0' + day;

        month = months[month];
        ostringstream ans;
        ans << year << "-" << month << "-" << day;
        return ans.str();
    }
};
