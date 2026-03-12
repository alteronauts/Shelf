#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Shelf"
BUNDLE_ID="io.helppi.shelf"
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
BINARY_NAME="shelf"

APP_BUNDLE="${APP_NAME}.app"
CONTENTS="${APP_BUNDLE}/Contents"
MACOS_DIR="${CONTENTS}/MacOS"
RESOURCES_DIR="${CONTENTS}/Resources"

# --- Code signing identity ---
# Set SIGN_IDENTITY to your "Developer ID Application: ..." certificate name
# for full Gatekeeper compliance. Falls back to ad-hoc signing (-) if unset.
SIGN_IDENTITY="${SIGN_IDENTITY:-}"

# --- Build universal binary (arm64 + x86_64) ---
echo "==> Building ${APP_NAME} v${VERSION} (release, universal)..."

cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

echo "==> Creating universal binary with lipo..."
lipo -create \
    "target/aarch64-apple-darwin/release/${BINARY_NAME}" \
    "target/x86_64-apple-darwin/release/${BINARY_NAME}" \
    -output "target/release/${BINARY_NAME}-universal"

echo "==> Creating ${APP_BUNDLE}..."
rm -rf "${APP_BUNDLE}"
mkdir -p "${MACOS_DIR}" "${RESOURCES_DIR}"

cp "target/release/${BINARY_NAME}-universal" "${MACOS_DIR}/${APP_NAME}"
chmod +x "${MACOS_DIR}/${APP_NAME}"

# --- Copy bundled fonts ---
if [ -d "fonts" ]; then
    echo "==> Bundling fonts..."
    cp -R fonts "${RESOURCES_DIR}/fonts"
fi

# --- Generate Info.plist ---
cat > "${CONTENTS}/Info.plist" << PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
</dict>
</plist>
PLIST

# --- Generate icon ---
ICON_SRC="assets/icon.png"
if [ -f "${ICON_SRC}" ]; then
    echo "==> Generating app icon from ${ICON_SRC}..."
    ICONSET="AppIcon.iconset"
    mkdir -p "${ICONSET}"
    sips -z 16 16     "${ICON_SRC}" --out "${ICONSET}/icon_16x16.png"      > /dev/null 2>&1
    sips -z 32 32     "${ICON_SRC}" --out "${ICONSET}/icon_16x16@2x.png"   > /dev/null 2>&1
    sips -z 32 32     "${ICON_SRC}" --out "${ICONSET}/icon_32x32.png"      > /dev/null 2>&1
    sips -z 64 64     "${ICON_SRC}" --out "${ICONSET}/icon_32x32@2x.png"   > /dev/null 2>&1
    sips -z 128 128   "${ICON_SRC}" --out "${ICONSET}/icon_128x128.png"    > /dev/null 2>&1
    sips -z 256 256   "${ICON_SRC}" --out "${ICONSET}/icon_128x128@2x.png" > /dev/null 2>&1
    sips -z 256 256   "${ICON_SRC}" --out "${ICONSET}/icon_256x256.png"    > /dev/null 2>&1
    sips -z 512 512   "${ICON_SRC}" --out "${ICONSET}/icon_256x256@2x.png" > /dev/null 2>&1
    sips -z 512 512   "${ICON_SRC}" --out "${ICONSET}/icon_512x512.png"    > /dev/null 2>&1
    sips -z 1024 1024 "${ICON_SRC}" --out "${ICONSET}/icon_512x512@2x.png" > /dev/null 2>&1
    iconutil -c icns "${ICONSET}" -o "${RESOURCES_DIR}/AppIcon.icns"
    rm -rf "${ICONSET}"
    /usr/libexec/PlistBuddy -c "Add :CFBundleIconFile string AppIcon" "${CONTENTS}/Info.plist"
else
    echo "    (No ${ICON_SRC} found — skipping icon generation)"
fi

# --- Code signing ---
if [ -n "${SIGN_IDENTITY}" ]; then
    echo "==> Signing with identity: ${SIGN_IDENTITY}"
    codesign --force --options runtime --timestamp \
        --sign "${SIGN_IDENTITY}" \
        "${APP_BUNDLE}"
else
    echo "==> Ad-hoc signing (no Developer ID certificate found)..."
    echo "    Set SIGN_IDENTITY env var for proper signing."
    codesign --force --deep -s - "${APP_BUNDLE}"
fi

# --- Verify ---
echo "==> Verifying code signature..."
codesign --verify --verbose=2 "${APP_BUNDLE}" 2>&1 || true

# --- Print binary info ---
SIZE=$(du -sh "${MACOS_DIR}/${APP_NAME}" | cut -f1)
ARCHS=$(lipo -info "${MACOS_DIR}/${APP_NAME}" 2>&1)
echo "==> Done! ${APP_BUNDLE} (${SIZE})"
echo "    Architectures: ${ARCHS}"

# --- Create DMG ---
DMG_NAME="${APP_NAME}-${VERSION}.dmg"
echo "==> Creating ${DMG_NAME}..."
rm -f "${DMG_NAME}"

if command -v create-dmg &> /dev/null; then
    create-dmg \
        --volname "${APP_NAME}" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "${APP_BUNDLE}" 150 190 \
        --app-drop-link 450 190 \
        "${DMG_NAME}" \
        "${APP_BUNDLE}"
else
    echo "    create-dmg not found, using hdiutil..."
    hdiutil create -volname "${APP_NAME}" \
        -srcfolder "${APP_BUNDLE}" \
        -ov -format UDZO \
        "${DMG_NAME}"
fi

# --- Sign the DMG too ---
if [ -n "${SIGN_IDENTITY}" ]; then
    echo "==> Signing DMG..."
    codesign --force --sign "${SIGN_IDENTITY}" "${DMG_NAME}"
fi

# --- Notarize (requires Developer ID + App Store Connect API key) ---
if [ -n "${SIGN_IDENTITY}" ] && [ -n "${NOTARIZE_TEAM_ID:-}" ]; then
    echo "==> Submitting for notarization..."
    xcrun notarytool submit "${DMG_NAME}" \
        --team-id "${NOTARIZE_TEAM_ID}" \
        --wait \
        --timeout 600

    echo "==> Stapling notarization ticket..."
    xcrun stapler staple "${DMG_NAME}"
    echo "==> Notarization complete!"
else
    if [ -n "${SIGN_IDENTITY}" ]; then
        echo ""
        echo "    Skipping notarization (set NOTARIZE_TEAM_ID to enable)."
        echo "    Run manually:"
        echo "      xcrun notarytool submit ${DMG_NAME} --apple-id YOUR_EMAIL --team-id YOUR_TEAM_ID --wait"
        echo "      xcrun stapler staple ${DMG_NAME}"
    fi
fi

DMG_SIZE=$(du -sh "${DMG_NAME}" | cut -f1)
echo ""
echo "==> Done! ${DMG_NAME} (${DMG_SIZE})"
echo "    Run:     open \"${APP_BUNDLE}\""
echo "    Install: open \"${DMG_NAME}\""

if [ -z "${SIGN_IDENTITY}" ]; then
    echo ""
    echo "    App is ad-hoc signed. Recipients must run:"
    echo "      xattr -cr ${APP_NAME}.app"
    echo "    before opening, or right-click > Open > Open."
fi
