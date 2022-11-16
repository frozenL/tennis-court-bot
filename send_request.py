from flask import Flask, request
from twilio.twiml.messaging_response import MessagingResponse

# useless

app = Flask(__name__)

@app.route("/", methods=["POST"])
# chatbot logic
def bot():

    # user input
    user_msg = request.values.get('Body', '').lower()

    # creating object of MessagingResponse
    response = MessagingResponse()

    msgs = [response.message(f"--- Result for '{user_msg}' are ---")]
    # searching and storing urls
    for i in range(0, 5):
        msgs.append(response.message(f"oui {i}"))

    return str(response)


if __name__ == "__main__":
	app.run()

