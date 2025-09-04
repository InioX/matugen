import fs from "fs";
import { QuantizerCelebi, Score, Scheme, argbFromRgb } from "@material/material-color-utilities";
import sharp from "sharp";
import { execSync } from "child_process";

async function extractOfficialMaterialYou(imagePath: string) {
    // Load image with sharp
    const image = sharp(imagePath).raw().ensureAlpha().toColorspace("srgb");
    const { data } = await image.toBuffer({ resolveWithObject: true });

    // Convert to ARGB pixels
    const pixels: number[] = [];
    for (let i = 0; i < data.length; i += 4) { // 4 bytes per pixel (RGBA)
        const r = data[i];
        const g = data[i + 1];
        const b = data[i + 2];
        pixels.push(argbFromRgb(r, g, b));
    }

    // Quantize colors and get primary
    const quantized = QuantizerCelebi.quantize(pixels, 128);
    const ranked = Score.score(quantized);
    const primary = ranked[0];

    // Generate official schemes
    const schemes = {
        light: Scheme.light(primary),
        dark: Scheme.dark(primary),
    };

    return schemes;
}

function argbToHex(argb: number): string {
    return '#' + (argb >>> 0).toString(16).padStart(8, '0').slice(2).toLowerCase();
}

function formatSchemeForComparison(scheme: any, label: string) {
    return {
        [label]: {
            background: typeof scheme.background === 'number' ? argbToHex(scheme.background) : scheme.background,
            surface: typeof scheme.surface === 'number' ? argbToHex(scheme.surface) : scheme.surface,
            on_surface: typeof scheme.onSurface === 'number' ? argbToHex(scheme.onSurface) : scheme.on_surface,
            on_surface_variant: typeof scheme.onSurfaceVariant === 'number' ? argbToHex(scheme.onSurfaceVariant) : scheme.on_surface_variant,
            primary: typeof scheme.primary === 'number' ? argbToHex(scheme.primary) : scheme.primary,
            on_primary: typeof scheme.onPrimary === 'number' ? argbToHex(scheme.onPrimary) : scheme.on_primary,
            primary_container: typeof scheme.primaryContainer === 'number' ? argbToHex(scheme.primaryContainer) : scheme.primary_container,
            on_primary_container: typeof scheme.onPrimaryContainer === 'number' ? argbToHex(scheme.onPrimaryContainer) : scheme.on_primary_container,
            secondary: typeof scheme.secondary === 'number' ? argbToHex(scheme.secondary) : scheme.secondary,
            on_secondary: typeof scheme.onSecondary === 'number' ? argbToHex(scheme.onSecondary) : scheme.on_secondary,
            secondary_container: typeof scheme.secondaryContainer === 'number' ? argbToHex(scheme.secondaryContainer) : scheme.secondary_container,
            tertiary: typeof scheme.tertiary === 'number' ? argbToHex(scheme.tertiary) : scheme.tertiary,
            on_tertiary: typeof scheme.onTertiary === 'number' ? argbToHex(scheme.onTertiary) : scheme.on_tertiary,
            tertiary_container: typeof scheme.tertiaryContainer === 'number' ? argbToHex(scheme.tertiaryContainer) : scheme.tertiary_container,
            error: typeof scheme.error === 'number' ? argbToHex(scheme.error) : scheme.error,
            on_error: typeof scheme.onError === 'number' ? argbToHex(scheme.onError) : scheme.on_error,
            error_container: typeof scheme.errorContainer === 'number' ? argbToHex(scheme.errorContainer) : scheme.error_container,
            outline: typeof scheme.outline === 'number' ? argbToHex(scheme.outline) : scheme.outline,
            outline_variant: typeof scheme.outlineVariant === 'number' ? argbToHex(scheme.outlineVariant) : scheme.outline_variant,
            surface_variant: typeof scheme.surfaceVariant === 'number' ? argbToHex(scheme.surfaceVariant) : scheme.surface_variant,
        }
    };
}

async function main() {
    const wallpaper = process.argv[2];
    const schemeType = process.argv[3] || 'scheme-tonal-spot';

    if (!wallpaper) {
        console.error("Usage: tsx material-you-comparison.ts <wallpaper.png> [scheme-type]");
        console.error("Available scheme types: scheme-tonal-spot, scheme-content, scheme-expressive, etc.");
        process.exit(1);
    }

    // Get official Material You colors
    const officialSchemes = await extractOfficialMaterialYou(wallpaper);

    // Get matugen colors
    let matugenJson: any = {};
    try {
        const raw = execSync(`./target/release/matugen image "${wallpaper}" -t ${schemeType} --json hex --dry-run --quiet`, { encoding: "utf8" });
        // Remove the "ok" at the end of the output
        const cleanedRaw = raw.trim().replace(/ok$/, '');
        matugenJson = JSON.parse(cleanedRaw);
    } catch (e: any) {
        console.error("Matugen failed:", e.message);
        console.error("This might be due to a malformed JSON output from matugen.");
        console.error("Please ensure you're using the latest version of matugen.");
        process.exit(1);
    }

    // Format for comparison
    const comparison = {
        image: wallpaper,
        scheme_type: schemeType,
        dark_mode: {
            ...formatSchemeForComparison(officialSchemes.dark, 'official_material_you'),
            matugen: {
                background: matugenJson.colors.background?.dark,
                surface: matugenJson.colors.surface?.dark,
                on_surface: matugenJson.colors.on_surface?.dark,
                on_surface_variant: matugenJson.colors.on_surface_variant?.dark,
                primary: matugenJson.colors.primary?.dark,
                on_primary: matugenJson.colors.on_primary?.dark,
                primary_container: matugenJson.colors.primary_container?.dark,
                on_primary_container: matugenJson.colors.on_primary_container?.dark,
                secondary: matugenJson.colors.secondary?.dark,
                on_secondary: matugenJson.colors.on_secondary?.dark,
                secondary_container: matugenJson.colors.secondary_container?.dark,
                tertiary: matugenJson.colors.tertiary?.dark,
                on_tertiary: matugenJson.colors.on_tertiary?.dark,
                tertiary_container: matugenJson.colors.tertiary_container?.dark,
                error: matugenJson.colors.error?.dark,
                on_error: matugenJson.colors.on_error?.dark,
                error_container: matugenJson.colors.error_container?.dark,
                outline: matugenJson.colors.outline?.dark,
                outline_variant: matugenJson.colors.outline_variant?.dark,
                surface_variant: matugenJson.colors.surface_variant?.dark,
            }
        },
        light_mode: {
            ...formatSchemeForComparison(officialSchemes.light, 'official_material_you'),
            matugen: {
                background: matugenJson.colors.background?.light,
                surface: matugenJson.colors.surface?.light,
                on_surface: matugenJson.colors.on_surface?.light,
                on_surface_variant: matugenJson.colors.on_surface_variant?.light,
                primary: matugenJson.colors.primary?.light,
                on_primary: matugenJson.colors.on_primary?.light,
                primary_container: matugenJson.colors.primary_container?.light,
                on_primary_container: matugenJson.colors.on_primary_container?.light,
                secondary: matugenJson.colors.secondary?.light,
                on_secondary: matugenJson.colors.on_secondary?.light,
                secondary_container: matugenJson.colors.secondary_container?.light,
                tertiary: matugenJson.colors.tertiary?.light,
                on_tertiary: matugenJson.colors.on_tertiary?.light,
                tertiary_container: matugenJson.colors.tertiary_container?.light,
                error: matugenJson.colors.error?.light,
                on_error: matugenJson.colors.on_error?.light,
                error_container: matugenJson.colors.error_container?.light,
                outline: matugenJson.colors.outline?.light,
                outline_variant: matugenJson.colors.outline_variant?.light,
                surface_variant: matugenJson.colors.surface_variant?.light,
            }
        }
    };

    // Quick console comparison for key colors
    const darkOfficial = comparison.dark_mode.official_material_you;
    const darkMatugen = comparison.dark_mode.matugen;

    console.log(`primary:        ${darkOfficial.primary} (official) vs ${darkMatugen.primary} (matugen)`);
    console.log(`on_surface:     ${darkOfficial.on_surface} (official) vs ${darkMatugen.on_surface} (matugen)`);
    console.log(`outline:        ${darkOfficial.outline} (official) vs ${darkMatugen.outline} (matugen)`);
    console.log(`surface:        ${darkOfficial.surface} (official) vs ${darkMatugen.surface} (matugen)`);

    // Save full comparison
    const filename = `material-comparison-${schemeType.replace('scheme-', '')}.json`;
    fs.writeFileSync(filename, JSON.stringify(comparison, null, 2));
    console.log(`Full comparison saved to: ${filename}`);
}

main();
