use egui::*;
use plot::{
    Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Plot, Legend
};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


pub struct TemplateApp {

    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPane\l`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        use egui::*;


        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            menu::bar(ui, |ui| {

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.menu_button("File", |ui| {
                    if ui.button("Order").clicked() {
                        ui.ctx().memory().reset_areas();
                        ui.close_menu();
                    }
                   
                });


            });

        });


        SidePanel::left("Host List")
        .default_width(200.0)
        .show(ctx, |ui| {

            for i in 1..60 {
                if i % 2 == 0 {
                    ui.toggle_value(&mut true, "host".to_owned()+ &i.to_string());
                } else {
                    ui.toggle_value(&mut false, "host".to_owned() + &i.to_string());
                }
                
            }

   });


        CentralPanel::default().show(ctx, |_ui| {


            TopBottomPanel::top("box plot panel").default_height(300.0).show_inside(_ui, |ui| {
                let box1 =  BoxPlot::new(vec![
                    BoxElem::new(0.5, BoxSpread::new(1.5, 1.8, 2.3, 2.6,3.0)).box_width(10.0).name("Day 1"),
                ])
                    .name("Experiment A")
                    .horizontal();

                Plot::new("Box Plot Demo")
                    .legend(Legend::default())
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.box_plot(box1);

                    }).response;
                
                });

                CentralPanel::default().show_inside(_ui, |ui| {
                        Window::new("Histogram").open(&mut true)
                        .enabled(true)
                        .vscroll(false)
                        .resizable(true)
                        .default_size([750.0, 550.0])
                        .show(ctx, |ui| {
                            CentralPanel::default().show_inside(ui, |ui| {
                                let chart = BarChart::new(
                                    (-395..=395)
                                        .step_by(10)
                                        .map(|x| x as f64 * 0.01)
                                        .map(|x| {
                                            (
                                                x,
                                                (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                                            )
                                        })
                                        // The 10 factor here is purely for a nice 1:1 aspect ratio
                                        .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
                                        .collect(),
                                )
                                    .color(Color32::LIGHT_BLUE)
                                    .name("Normal Distribution");
        
                                Plot::new("Normal Distribution Demo")
                                    .legend(Legend::default())
                                    .data_aspect(1.0)
                                    .show(ui, |plot_ui| plot_ui.bar_chart(chart)).response;
                            });
                            ui.allocate_space(ui.available_size());
                            ui.close_menu();
                        });
        
        
                    Window::new("Stacked Bar Chart")
                        .vscroll(false)
                        .resizable(true)
                        .default_size([750.0, 550.0])
                        .show(ctx, |ui| {
                            CentralPanel::default().show_inside(ui, |ui| {
                                let chart1 = BarChart::new(vec![
                                    Bar::new(0.5, 1.0).name("Day 1"),
                                    Bar::new(1.5, 3.0).name("Day 2"),
                                    Bar::new(2.5, 1.0).name("Day 3"),
                                    Bar::new(3.5, 2.0).name("Day 4"),
                                    Bar::new(4.5, 4.0).name("Day 5"),
                                ])
                                    .width(0.7)
                                    .name("Set 1");
        
                                let chart2 = BarChart::new(vec![
                                    Bar::new(0.5, 1.0),
                                    Bar::new(1.5, 1.5),
                                    Bar::new(2.5, 0.1),
                                    Bar::new(3.5, 0.7),
                                    Bar::new(4.5, 0.8),
                                ])
                                    .width(0.7)
                                    .name("Set 2")
                                    .stack_on(&[&chart1]);
        
                                let chart3 = BarChart::new(vec![
                                    Bar::new(0.5, -0.5),
                                    Bar::new(1.5, 1.0),
                                    Bar::new(2.5, 0.5),
                                    Bar::new(3.5, -1.0),
                                    Bar::new(4.5, 0.3),
                                ])
                                    .width(0.7)
                                    .name("Set 3")
                                    .stack_on(&[&chart1, &chart2]);
        
                                let chart4 = BarChart::new(vec![
                                    Bar::new(0.5, 0.5),
                                    Bar::new(1.5, 1.0),
                                    Bar::new(2.5, 0.5),
                                    Bar::new(3.5, -0.5),
                                    Bar::new(4.5, -0.5),
                                ])
                                    .width(0.7)
                                    .name("Set 4")
                                    .stack_on(&[&chart1, &chart2, &chart3]);
        
                                Plot::new("Stacked Bar Chart Demo")
                                    .legend(Legend::default())
                                    .data_aspect(1.0)
                                    .show(ui, |plot_ui| {
                                        plot_ui.bar_chart(chart1);
                                        plot_ui.bar_chart(chart2);
                                        plot_ui.bar_chart(chart3);
                                        plot_ui.bar_chart(chart4);
                                    }).response;
                            });
                            ui.allocate_space(ui.available_size());
                            ui.close_menu();
                        });
                    });

                TopBottomPanel::bottom("host grid").default_height(500.0).show_inside(_ui, |ui| {

                    Grid::new("some_unique_id")
                    .num_columns(6).max_col_width(500.0).max_col_width(500.0).min_row_height(10.0).show(ui, |ui| {

                        for _n in 1..10 {
                            ui.label("1 column");
                            ui.label("2 column");
                            ui.label("3 column");
                            ui.label("4 column");
                            ui.label("5 column");
                            ui.label("6 column");
                            ui.end_row();
                        }
          
                    });
    
    
                })
                

        });


       
        
        // CentralPanel::default().show(ctx, |_ui| {

        //     TopBottomPanel::top("box plot panel").show_inside(_ui, |ui| {
                
        //         CentralPanel::default().show_inside(ui, |ui| {
        //             let yellow = Color32::from_rgb(248, 252, 168);
        //             let box1 =  BoxPlot::new(vec![
        //                 BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
        //             ])
        //                 .name("Experiment A")
        //                 .vertical();

        //             Plot::new("Box Plot Demo")
        //                 .legend(Legend::default())
        //                 .show(ui, |plot_ui| {
        //                     plot_ui.box_plot(box1);

        //                 }).response;
        //         });

        //     });

        // });

            // SidePanel::left("Host List")
            // .default_width(200.0)
            // .show(ctx, |ui| {
            //     _ui.toggle_value(&mut false, "host1");
            //     _ui.toggle_value(&mut false, "host2");
            //     _ui.toggle_value(&mut false, "host3");
            //     _ui.toggle_value(&mut false, "host4");
            //     _ui.toggle_value(&mut false, "host5");
            //     _ui.toggle_value(&mut false, "host6");
            //     _ui.toggle_value(&mut false, "host7");
            //     _ui.toggle_value(&mut false, "host8");
            //     _ui.toggle_value(&mut false, "host9");
            //     _ui.toggle_value(&mut false, "host10");

            // });

            // CentralPanel::default().show(ctx, |ui| {
            //     Window::new("Histogram").open(&mut true)
            //     .enabled(true)
            //     .vscroll(false)
            //     .resizable(true)
            //     .default_size([750.0, 550.0])
            //     .show(ctx, |ui| {
            //         CentralPanel::default().show_inside(ui, |ui| {
            //             let chart = BarChart::new(
            //                 (-395..=395)
            //                     .step_by(10)
            //                     .map(|x| x as f64 * 0.01)
            //                     .map(|x| {
            //                         (
            //                             x,
            //                             (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
            //                         )
            //                     })
            //                     // The 10 factor here is purely for a nice 1:1 aspect ratio
            //                     .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
            //                     .collect(),
            //             )
            //                 .color(Color32::LIGHT_BLUE)
            //                 .name("Normal Distribution");

            //             Plot::new("Normal Distribution Demo")
            //                 .legend(Legend::default())
            //                 .data_aspect(1.0)
            //                 .show(ui, |plot_ui| plot_ui.bar_chart(chart)).response;
            //         });
            //         ui.allocate_space(ui.available_size());
            //         ui.close_menu();
            //     });


            // Window::new("Stacked Bar Chart")
            //     .vscroll(false)
            //     .resizable(true)
            //     .default_size([750.0, 550.0])
            //     .show(ctx, |ui| {
            //         CentralPanel::default().show_inside(ui, |ui| {
            //             let chart1 = BarChart::new(vec![
            //                 Bar::new(0.5, 1.0).name("Day 1"),
            //                 Bar::new(1.5, 3.0).name("Day 2"),
            //                 Bar::new(2.5, 1.0).name("Day 3"),
            //                 Bar::new(3.5, 2.0).name("Day 4"),
            //                 Bar::new(4.5, 4.0).name("Day 5"),
            //             ])
            //                 .width(0.7)
            //                 .name("Set 1");

            //             let chart2 = BarChart::new(vec![
            //                 Bar::new(0.5, 1.0),
            //                 Bar::new(1.5, 1.5),
            //                 Bar::new(2.5, 0.1),
            //                 Bar::new(3.5, 0.7),
            //                 Bar::new(4.5, 0.8),
            //             ])
            //                 .width(0.7)
            //                 .name("Set 2")
            //                 .stack_on(&[&chart1]);

            //             let chart3 = BarChart::new(vec![
            //                 Bar::new(0.5, -0.5),
            //                 Bar::new(1.5, 1.0),
            //                 Bar::new(2.5, 0.5),
            //                 Bar::new(3.5, -1.0),
            //                 Bar::new(4.5, 0.3),
            //             ])
            //                 .width(0.7)
            //                 .name("Set 3")
            //                 .stack_on(&[&chart1, &chart2]);

            //             let chart4 = BarChart::new(vec![
            //                 Bar::new(0.5, 0.5),
            //                 Bar::new(1.5, 1.0),
            //                 Bar::new(2.5, 0.5),
            //                 Bar::new(3.5, -0.5),
            //                 Bar::new(4.5, -0.5),
            //             ])
            //                 .width(0.7)
            //                 .name("Set 4")
            //                 .stack_on(&[&chart1, &chart2, &chart3]);

            //             Plot::new("Stacked Bar Chart Demo")
            //                 .legend(Legend::default())
            //                 .data_aspect(1.0)
            //                 .show(ui, |plot_ui| {
            //                     plot_ui.bar_chart(chart1);
            //                     plot_ui.bar_chart(chart2);
            //                     plot_ui.bar_chart(chart3);
            //                     plot_ui.bar_chart(chart4);
            //                 }).response;
            //         });
            //         ui.allocate_space(ui.available_size());
            //         ui.close_menu();
            //     });


            // });

            
            
       


    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

}