from manim import *

class GenScene(Scene):
    def construct(self):
        # === Setup Buckets as Containers ===
        self.bucket_containers = []
        self.values_map = {}

        for i in range(5):
            square = Square(side_length=1.5, color=WHITE)
            label = Text(f"t+{i}s", font_size=20, color=RED if i == 0 else WHITE).next_to(square, DOWN)
            container = VGroup(square, label).move_to(RIGHT * (i * 2.0))
            self.bucket_containers.append(container)

        self.bucket_group = VGroup(*self.bucket_containers).move_to(DOWN * 1)

        # Time marker
        self.current_time_marker = Triangle(color=YELLOW).scale(0.2).next_to(self.bucket_containers[0], UP)

        # Static labels
        self.timer = Text("t = 0.0s", font_size=28).to_corner(UR)
        self.function_call = Text("SkipListExpiry::new(num_buckets=5, resolution=1)", font_size=28).to_edge(UP)

        self.add(self.bucket_group, self.current_time_marker, self.timer, self.function_call)
        self.wait(0.5)

        # === Insert A ===
        self.insert("A", target_bucket=2, time="t = 1.0s", label_text='insert(id="A", close_time=now+2)')

        # === Insert B ===
        self.insert("B", target_bucket=3, time="t = 2.0s", label_text='insert(id="B", close_time=now+3)')

        # === values() before expiry ===
        self.show_values(["A", "B"], time="t = 3.0s")

        # === expire_front() 1 ===
        self.expire_front(time="t = 4.0s")

        # === values() again ===
        self.show_values(["A", "B"], time="t = 5.0s")

        # === expire_front() 2 — A expires ===
        self.expire_front(time="t = 6.0s", expired_value="A")

        # === values() after A expired ===
        self.show_values(["B"], time="t = 7.0s")

        # === expire_front() 3 — B expires ===
        self.expire_front(time="t = 8.0s", expired_value="B")

        # === values() -> [] ===
        self.show_values([], time="t = 9.0s")

        # === Done ===
        final_message = Text("All expired. Buckets are empty.", font_size=28).to_edge(DOWN)
        self.play(Write(final_message))
        self.wait(2)

    def insert(self, value_text, target_bucket, time, label_text):
        self.function_call.become(Text(label_text, font_size=28).to_edge(UP))
        self.timer.become(Text(time, font_size=28).to_corner(UR))

        value = Text(value_text, font_size=30, color=RED).next_to(self.bucket_containers[target_bucket], UP)
        self.add(value)
        self.play(value.animate.move_to(self.bucket_containers[target_bucket].get_center()))
        self.wait(0.5)
        self.play(value.animate.set_color(BLUE))
        self.bucket_containers[target_bucket].add(value)
        self.values_map[value_text] = value
        self.wait(0.5)

    def expire_front(self, time, expired_value=None):
        self.function_call.become(Text("expire_front()", font_size=28).to_edge(UP))
        self.timer.become(Text(time, font_size=28).to_corner(UR))

        # Highlight the front bucket
        self.play(Indicate(self.bucket_containers[0][0], color=RED))  # highlight the square only

        # Remove front bucket (with fade out)
        expired_bucket = self.bucket_containers.pop(0)
        self.play(FadeOut(expired_bucket))

        # Expire value visually if applicable
        if expired_value and expired_value in self.values_map:
            self.play(FadeOut(self.values_map[expired_value]))
            del self.values_map[expired_value]

        # Create a new bucket at the end
        old_label = expired_bucket[1].text  # e.g., 't+0s'
        old_num = int(old_label.split('+')[1].split('s')[0])
        new_num = old_num + 5

        new_square = Square(side_length=1.5, color=WHITE)
        new_label = Text(f"t+{new_num}s", font_size=20, color=WHITE).next_to(new_square, DOWN)
        new_container = VGroup(new_square, new_label)
        self.bucket_containers.append(new_container)

        # Rearrange all buckets horizontally
        self.bucket_group = VGroup(*self.bucket_containers).arrange(RIGHT, buff=1).move_to(DOWN * 1)
        self.play(*[bucket.animate.move_to(self.bucket_group[i].get_center()) for i, bucket in enumerate(self.bucket_containers)])

        # Move the time marker to new front
        self.play(self.current_time_marker.animate.next_to(self.bucket_containers[0], UP))

        # Update label colors
        for i, bucket in enumerate(self.bucket_containers):
            label = bucket[1]
            new_color = RED if i == 0 else WHITE
            self.play(label.animate.set_color(new_color), run_time=0.2)

    def show_values(self, values, time):
        self.function_call.become(Text(f'values() -> {values}', font_size=28).to_edge(UP))
        self.timer.become(Text(time, font_size=28).to_corner(UR))

        highlight_boxes = [bucket[0].copy().set_color(GREEN).set_opacity(0.3) for bucket in self.bucket_containers]
        self.play(*[FadeIn(box) for box in highlight_boxes])
        self.wait(1)
        self.play(*[FadeOut(box) for box in highlight_boxes])
