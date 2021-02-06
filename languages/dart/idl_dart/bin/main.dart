// @dart=2.9
import 'dart:convert';
import 'dart:io';

import 'package:idl_dart/native_idl.dart';

void sendResponse(LanguageResponse response) => print(jsonEncode(response));
LanguageRequest getRequest(String requestStr) =>
    LanguageRequest.fromJson(jsonDecode(requestStr) as Map<String, dynamic>);

void main(List<String> arguments) async {
  var resultStr = '';

  while (true) {
    final str = stdin.readLineSync();

    if (str == null) {
      break;
    } else {
      resultStr += str;
    }
  }

  try {
    final request = getRequest(resultStr);
    final module = Module(request);
    sendResponse(module.generateResponse());
  } catch (exception, stackTrace) {
    final response = LanguageResponse(
        responseMessages: [],
        genResponse: ResponseType(
            value: Undefined(
                undefined:
                    exception.toString() + '\n' + stackTrace.toString())));
    sendResponse(response);
  }
}
