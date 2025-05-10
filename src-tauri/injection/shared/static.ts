export const TOP_HTML = `
<!-- These SVGS where taken from https://www.iconpacks.net/ -->
<style>
  .topright {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: center;
  }

  #topclose,
  #topmin,
  #topmax {
    display: flex;
    align-items: center;
    justify-content: center;

    height: 32px;
    width: 44px;

    color: var(--icon-tertiary);

    transition: all 0.1s ease-in-out;
  }

  #topclose:hover,
  #topmin:hover,
  #topmax:hover {
    filter: brightness(0.8);
    background: var(--background-secondary);
  }

  #topmax svg {
    display: none;
  }

  #topclose svg,
  #topmin svg,
  #topmax svg {
    height: 10px !important;
    width: 10px !important;
  }

  #topmax:not(.maximized) #svgmax,
  #topmax.maximized #svgunmax {
    display: initial;
  }

  #topclose:hover {
    background: var(--status-danger);
    color: var(--white);
  }
</style>

<div id="dorion_topbar" class="topright">
  <div id="topmin">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      version="1.1"
      width="24"
      height="24"
      viewBox="0 0 256 256"
      xml:space="preserve"
    >
      <defs></defs>
      <g
        style="
          stroke: none;
          stroke-width: 0;
          stroke-dasharray: none;
          stroke-linecap: butt;
          stroke-linejoin: miter;
          stroke-miterlimit: 10;
          fill: none;
          fill-rule: nonzero;
          opacity: 1;
        "
        transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
      >
        <path
          d="M 86.5 48.5 h -83 C 1.567 48.5 0 46.933 0 45 s 1.567 -3.5 3.5 -3.5 h 83 c 1.933 0 3.5 1.567 3.5 3.5 S 88.433 48.5 86.5 48.5 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
        <path
          d="M 86.5 48.5 h -83 C 1.567 48.5 0 46.933 0 45 s 1.567 -3.5 3.5 -3.5 h 83 c 1.933 0 3.5 1.567 3.5 3.5 S 88.433 48.5 86.5 48.5 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
      </g>
    </svg>
  </div>

  <div id="topmax">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      version="1.1"
      width="24"
      height="24"
      viewBox="0 0 256 256"
      xml:space="preserve"
      id="svgmax"
    >
      <defs></defs>
      <g
        style="
          stroke: none;
          stroke-width: 0;
          stroke-dasharray: none;
          stroke-linecap: butt;
          stroke-linejoin: miter;
          stroke-miterlimit: 10;
          fill: none;
          fill-rule: nonzero;
          opacity: 1;
        "
        transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
      >
        <path
          d="M 90 90 H 0 V 0 h 90 V 90 z M 10 80 h 70 V 10 H 10 V 80 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
      </g>
    </svg>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      version="1.1"
      width="24"
      height="24"
      viewBox="0 0 256 256"
      xml:space="preserve"
      id="svgunmax"
    >
      <defs></defs>
      <g
        style="
          stroke: none;
          stroke-width: 0;
          stroke-dasharray: none;
          stroke-linecap: butt;
          stroke-linejoin: miter;
          stroke-miterlimit: 10;
          fill: none;
          fill-rule: nonzero;
          opacity: 1;
        "
        transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
      >
        <path
          d="M 70 90 h -70 v -70 h 70 v 70 z M 10 80 h 50 v -50 h -50 v 50 z M 20 0 H 90 V 70 H 80 V 10 H 20 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
      </g>
    </svg>
  </div>

  <div id="topclose">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      version="1.1"
      width="24"
      height="24"
      viewBox="0 0 256 256"
      xml:space="preserve"
    >
      <defs></defs>
      <g
        style="
          stroke: none;
          stroke-width: 0;
          stroke-dasharray: none;
          stroke-linecap: butt;
          stroke-linejoin: miter;
          stroke-miterlimit: 10;
          fill: none;
          fill-rule: nonzero;
          opacity: 1;
        "
        transform="translate(1.4065934065934016 1.4065934065934016) scale(2.81 2.81)"
      >
        <path
          d="M 8 90 c -2.047 0 -4.095 -0.781 -5.657 -2.343 c -3.125 -3.125 -3.125 -8.189 0 -11.314 l 74 -74 c 3.125 -3.124 8.189 -3.124 11.314 0 c 3.124 3.124 3.124 8.189 0 11.313 l -74 74 C 12.095 89.219 10.047 90 8 90 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
        <path
          d="M 82 90 c -2.048 0 -4.095 -0.781 -5.657 -2.343 l -74 -74 c -3.125 -3.124 -3.125 -8.189 0 -11.313 c 3.124 -3.124 8.189 -3.124 11.313 0 l 74 74 c 3.124 3.125 3.124 8.189 0 11.314 C 86.095 89.219 84.048 90 82 90 z"
          style="
            stroke: none;
            stroke-width: 1;
            stroke-dasharray: none;
            stroke-linecap: butt;
            stroke-linejoin: miter;
            stroke-miterlimit: 10;
            fill: currentColor;
            fill-rule: nonzero;
            opacity: 1;
          "
          transform=" matrix(1 0 0 1 0 0) "
          stroke-linecap="round"
        />
      </g>
    </svg>
  </div>
</div>
`