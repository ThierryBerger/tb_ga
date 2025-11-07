import UIKit
import GameAnalytics

@_cdecl("tb_ga_init")
public func tb_ga_init() {
    guard let ga_game_key: String? = Bundle.main.infoDictionary?["GA_GAME_KEY"] as? String else {
        print("Error: GA_GAME_KEY not found.")
        return;
    };
    guard let ga_secret_key: String? = Bundle.main.infoDictionary?["GA_SECRET_KEY"] as? String else {
        print("Error: GA_SECRET_KEY not found.")
        return;
    };
    DispatchQueue.main.async {
        print("Initializing GameAnalytics SDK");
        GameAnalytics.initialize(withGameKey: ga_game_key, gameSecret: ga_secret_key);
    }
}

@_cdecl("tb_ga_add_progression_event_p123_score")
public func tb_ga_add_progression_event_p123_score(_ progressionStatus: Int32, _ p01: UnsafePointer<CChar>, _ p02: UnsafePointer<CChar>, _ p03: UnsafePointer<CChar>, _ score: Int32) {
    
    let ga_progression = GAProgressionStatus(rawValue: Int(progressionStatus));
    let p01 = String(cString: p01)
    let p02 = String(cString: p02);
    let p03 = String(cString: p03);
    // TODO: add custom fields
    GameAnalytics.addProgressionEvent(
        with: ga_progression,
        progression01: p01,
        progression02: p02,
        progression03: p03,
        score: Int(score));
}

@_cdecl("tb_ga_add_progression_event_p12_score")
public func tb_ga_add_progression_event_p12_score(_ progressionStatus: Int32, _ p01: UnsafePointer<CChar>, _ p02: UnsafePointer<CChar>, _ score: Int32) {
    let ga_progression = GAProgressionStatus(rawValue: Int(progressionStatus));
    let p01 = String(cString: p01)
    let p02 = String(cString: p02);
    // TODO: add custom fields
    GameAnalytics.addProgressionEvent(
        with: ga_progression,
        progression01: p01,
        progression02: p02,
        progression03: nil,
        score: Int(score));
}

@_cdecl("tb_ga_add_progression_event_p12")
public func tb_ga_add_progression_event_p12(_ progressionStatus: Int32, _ p01: UnsafePointer<CChar>, _ p02: UnsafePointer<CChar>) {
    let ga_progression = GAProgressionStatus(rawValue: Int(progressionStatus));
    let p01 = String(cString: p01)
    let p02 = String(cString: p02);
    // TODO: add custom fields
    GameAnalytics.addProgressionEvent(
        with: ga_progression,
        progression01: p01,
        progression02: p02,
        progression03: nil);
}

@_cdecl("tb_ga_add_business_event")
public func tb_ga_add_business_event(_ currency: UnsafePointer<CChar>, _ amount: Int32, _ item_id: UnsafePointer<CChar>, _ item_type: UnsafePointer<CChar>, _ cart_type: UnsafePointer<CChar>) {
    let currency = String(cString: currency)
    let item_id = String(cString: item_id);
    let item_type = String(cString: item_type);
    let cart_type = String(cString: cart_type);
    
    GameAnalytics.addBusinessEvent(
        withCurrency: currency,
        amount: Int(amount),
        itemType: item_type,
        itemId: item_id,
        cartType: cart_type,
        // TODO: be able to pass a receipt from Rust.
        autoFetchReceipt: true);
}

@_cdecl("tb_ga_add_design_event_with_value")
public func tb_ga_add_design_event_with_value(_ event_id: UnsafePointer<CChar>, _ value: Float) {
    let event_id = String(cString: event_id)

    GameAnalytics.addDesignEvent(
        withEventId: event_id,
        value: NSNumber(value: value)
    );
}

@_cdecl("tb_ga_add_design_event")
public func tb_ga_add_design_event(_ event_id: UnsafePointer<CChar>) {
    let event_id = String(cString: event_id)

    GameAnalytics.addDesignEvent(
        withEventId: event_id
    );
}
