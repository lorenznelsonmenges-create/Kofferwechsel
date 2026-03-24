import json
import requests
import time

def add_test_orders():
    try:
        resp = requests.get('http://127.0.0.1:3000/api/state')
        data = resp.json()
        base = data['auftraege'][0].copy()
        
        test_data = [
            ('Feuerwehr Berlin', 'KW-2024-002', 38000.0, 'InArbeit'),
            ('DRK Hamburg', 'KW-2024-003', 42000.0, 'Angenommen'),
            ('Malteser München', 'KW-2024-004', 55000.0, 'Bereitstellung'),
            ('Johanniter Köln', 'KW-2024-005', 47500.0, 'InArbeit'),
            ('Rettungsdienst Leipzig', 'KW-2024-006', 49000.0, 'Abgeschlossen')
        ]
        
        new_auftraege = [data['auftraege'][0]]
        for name, nr, umsatz, status in test_data:
            a = base.copy()
            a['auftraggeber'] = {'name': name, 'kontakt': f'info@{name.lower().replace(" ", "-")}.de'}
            a['auftrags_nummer'] = nr
            a['umsatz'] = umsatz
            a['status'] = status
            new_auftraege.append(a)
            
        data['auftraege'] = new_auftraege
        requests.post('http://127.0.0.1:3000/api/state', json=data)
        print("5 Test-Aufträge erfolgreich hinzugefügt.")
    except Exception as e:
        print(f"Fehler: {e}")

if __name__ == "__main__":
    add_test_orders()
