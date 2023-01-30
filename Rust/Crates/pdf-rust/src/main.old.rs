extern crate printpdf;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;
use std::thread::current;

fn print_circle() {
    use printpdf::utils::{calculate_points_for_circle, calculate_points_for_rect};

    let (doc, page1, layer1) = PdfDocument::new("printpdf circle test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let radius = Pt(50.0);
    let offset_x = Pt(100.0);
    let offset_y = Pt(100.0);

    let line = Line { 
        points: calculate_points_for_circle(radius, offset_x, offset_y), 
        is_closed: true, 
        has_fill: true, 
        has_stroke: true,
        is_clipping_path: false
    };

    current_layer.add_shape(line);

    let scale_x_rect = Pt(100.0);
    let scale_y_rect = Pt(10.0);
    let offset_x_rect = Pt(100.0);
    let offset_y_rect = Pt(50.0);

    let line = Line { 
        points: calculate_points_for_rect(scale_x_rect, scale_y_rect, offset_x_rect, offset_y_rect), 
        is_closed: true, 
        has_fill: true, 
        has_stroke: true,
        is_clipping_path: false
    };

    current_layer.add_shape(line);

    doc.save(&mut BufWriter::new(File::create("test_circle.pdf").unwrap())).unwrap();
}

fn print_graphics() {
    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(297.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![(Point::new(Mm(100.0), Mm(100.0)), false),
                       (Point::new(Mm(100.0), Mm(200.0)), false),
                       (Point::new(Mm(300.0), Mm(200.0)), false),
                       (Point::new(Mm(300.0), Mm(100.0)), false)];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line { 
        points: points1, 
        is_closed: true, 
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    // Triangle shape
    // Note: Line is invisible by default, the previous method of 
    // constructing a line is recommended!
    let mut line2 = Line::from_iter(vec![
        (Point::new(Mm(150.0), Mm(150.0)), false),
        (Point::new(Mm(150.0), Mm(250.0)), false),
        (Point::new(Mm(350.0), Mm(250.0)), false)]);

    line2.set_closed(false);
    line2.set_stroke(true);
    line2.set_fill(false);
    line2.set_as_clipping_path(false);

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(10.0);

    // Draw first line
    current_layer.add_shape(line1);
    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

    // More advanced graphical options
    current_layer.set_overprint_stroke(true);
    current_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
    current_layer.set_line_dash_pattern(dash_pattern);
    current_layer.set_line_cap_style(LineCapStyle::Round);
    current_layer.set_line_join_style(LineJoinStyle::Round);
    current_layer.set_fill_color(fill_color_2);
    current_layer.set_outline_color(outline_color_2);
    current_layer.set_outline_thickness(15.0);

    // draw second line
    current_layer.add_shape(line2);

    // If this is successful, you should see a PDF two shapes, one rectangle
    // and a dotted line
    doc.save(&mut BufWriter::new(File::create("test_graphics.pdf").unwrap())).unwrap();
}

fn print_invoice_square() {
    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(297.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let line1 = Line { 
        points: vec![(Point::new(Mm(10.0), Mm(10.0)), false),
                        (Point::new(Mm(10.0), Mm(200.0)), false)],
        is_closed: true, 
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    let line2 = Line {
        points: vec![(Point::new(Mm(10.0), Mm(200.0)), false),
                        (Point::new(Mm(287.0), Mm(200.0)), false)],
        is_closed: true, 
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    let text = "Lorem ipsum";
    let text_qte = "Qte";
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    current_layer.add_shape(line1);
    current_layer.add_shape(line2);
    current_layer.use_text(text, 12.0, Mm(12.0), Mm(196.0), &font);
    current_layer.use_text(text_qte, 12.0, Mm(200.0), Mm(201.0), &font);
    doc.save(&mut BufWriter::new(File::create("test_invoice_square.pdf").unwrap())).unwrap();
}

fn main() {
    // print_circle();
    // print_graphics();
    print_invoice_square();
}
