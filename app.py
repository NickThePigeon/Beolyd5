import telnetlib
import json

# HEOS device IP and port (replace with actual IP of your HEOS Link HS1)
HOST = "192.168.1.2"  # Update with the actual IP address of your HEOS device
PORT = 1255
NAME = "HEOS Woonkamer"
PID = 2087229988
MODEL = "HEOS Link"

# Telnet session function
def send_heos_command(command):
    try:
        # Establish telnet connection
        tn = telnetlib.Telnet(HOST, PORT)
        print(f"Connected to HEOS device at {HOST}:{PORT}")
        
        # Send command to HEOS (add newline to terminate command)
        tn.write(command.encode('ascii') + b'\r\n')
        
        # Read response from HEOS
        response = tn.read_until(b"\r\n").decode('ascii')
        print(f"Response from HEOS: {response}")
        
        # Try to parse response as JSON
        try:
            json_response = json.loads(response)
            return json_response
        except json.JSONDecodeError:
            print("Failed to decode JSON from response")
            return None
    except Exception as e:
        print(f"Error: {e}")
        return None
    finally:
        tn.close()

# Test command: Retrieve current volume level (you can replace this with other commands)
# command = 'heos://player/get_volume?pid=YOUR_PLAYER_ID'  # Replace YOUR_PLAYER_ID with your actual player ID

def enable_pretty_json():
    command = 'heos://system/prettify_json_response?enable=on'
    return send_heos_command(command=command)

def get_players():
    command = 'heos://player/get_players'
    return send_heos_command(command=command)

def get_player_info():
    command = f'heos://player/get_player_info?pid={PID}'
    return send_heos_command(command=command)

def get_player_state():
    command = f'heos://player/get_play_state?pid={PID}&state=play_state'
    return send_heos_command(command=command)

def set_player_state(state):
    if not state:
        print("No state argument")
        return None
    command = f'heos://player/set_play_state?pid={PID}&state={state}'
    return send_heos_command(command=command)

def set_volume(volume):
    if volume < 0 or volume > 100:
        print("Volume out of range")
        return None
    command = f"heos://player/set_volume?pid={PID}&level={volume}"
    return send_heos_command(command=command)

def get_volume():
    command = f'heos://player/get_volume?pid={PID}'
    return send_heos_command(command=command)

def volume_up(step=5):
    command = f'heos://player/volume_up?pid={PID}&step={step}'
    return send_heos_command(command=command)

def volume_down(step=5):
    command = f'heos://player/volume_down?pid={PID}&step={step}'
    return send_heos_command(command=command)

def get_mute():
    command = f'heos://player/get_mute?pid={PID}'
    return send_heos_command(command=command)

def set_mute(state: bool):
    if state:
        toggle = 'on'
    else:
        toggle = 'off'
    command = f'heos://player/set_mute?pid={PID}&state={toggle}'
    return send_heos_command(command=command)

def toggle_mute():
    command = f'heos://player/toggle_mute?pid={PID}'
    return send_heos_command(command=command)

def get_sources():
    command = f'heos://browse/get_music_sources'
    return send_heos_command(command=command)

def browse_source(sid=4):
    """
    Aux input sid: 1027
    Spotify sid: 4
    """
    command = f'heos://browse/browse?sid={sid}'
    return send_heos_command(command=command)


def set_source_input_tv():
    command = f"heos://browse/play_input?pid={PID}&sid=1027&input=inputs/optical_in_1"
    return send_heos_command(command=command)

def set_source_input_spotify():
    command = f"heos://browse/play_input?pid={PID}&sid=5"
    return send_heos_command(command=command)

def heartbeat():
    return send_heos_command('heos://system/heart_beat')

def get_now_playing():
    command = f"heos://player/get_now_playing_media?pid={PID}"
    return send_heos_command(command=command)

# def play_spotify_mid():
#     command = f'heos://browse/play_stream?pid={PID}&sid=4&mid=spotify:track:5z6xHjCZr7a7AIcy8sPBKy'
#     return send_heos_command(command=command)
 

# If i stream music with spotify via pc the sid == 4 and qid == 1
# For aux sid == 1027

if __name__ == '__main__':
    response = get_players()
    if response:
        print("Parsed JSON response:")

