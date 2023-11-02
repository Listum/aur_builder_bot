FROM archlinux:base-devel
RUN pacman -Syuu --noconfirm
WORKDIR /opt/aur_builder
RUN useradd -m -g users -G wheel -s /bin/bash aur && echo "%wheel ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers
ADD https://git.orudo.ru/OrudoCA/aur_builder_bot/releases/download/0.1.0/aur_builder_bot /opt/aur_builder/aur_builder_bot
RUN chmod +x aur_builder_bot && ln -sf /opt/aur_builder/aur_builder_bot /usr/bin/aur_builder_bot && chown aur -R /opt/aur_builder
USER aur
ENTRYPOINT ["aur_builder_bot"]
