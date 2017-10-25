use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 113, 227, 37], OperandSize::Dword)
}

fn vpsraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 113, 228, 62], OperandSize::Qword)
}

fn vpsraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 229, 109], OperandSize::Dword)
}

fn vpsraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 113, 230, 93], OperandSize::Qword)
}

fn vpsraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 113, 224, 7], OperandSize::Dword)
}

fn vpsraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 113, 39, 3], OperandSize::Dword)
}

fn vpsraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 77, 142, 113, 231, 21], OperandSize::Qword)
}

fn vpsraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 133, 113, 36, 193, 55], OperandSize::Qword)
}

fn vpsraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 113, 228, 126], OperandSize::Dword)
}

fn vpsraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 173, 113, 36, 190, 19], OperandSize::Dword)
}

fn vpsraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM30)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 125, 163, 113, 230, 113], OperandSize::Qword)
}

fn vpsraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 164, 113, 36, 242, 68], OperandSize::Qword)
}

fn vpsraw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 202, 113, 229, 24], OperandSize::Dword)
}

fn vpsraw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1240308188, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 113, 36, 181, 220, 153, 237, 73, 96], OperandSize::Dword)
}

fn vpsraw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 37, 193, 113, 224, 124], OperandSize::Qword)
}

fn vpsraw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 53, 196, 113, 36, 82, 125], OperandSize::Qword)
}

fn vpsraw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 225, 250], OperandSize::Dword)
}

fn vpsraw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 225, 30], OperandSize::Dword)
}

fn vpsraw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 225, 195], OperandSize::Qword)
}

fn vpsraw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 225, 60, 66], OperandSize::Qword)
}

fn vpsraw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 225, 234], OperandSize::Dword)
}

fn vpsraw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 225, 12, 86], OperandSize::Dword)
}

fn vpsraw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 225, 227], OperandSize::Qword)
}

fn vpsraw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 225, 28, 176], OperandSize::Qword)
}

fn vpsraw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 225, 198], OperandSize::Dword)
}

fn vpsraw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 225, 36, 118], OperandSize::Dword)
}

fn vpsraw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 37, 132, 225, 224], OperandSize::Qword)
}

fn vpsraw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 2137690853, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 13, 141, 225, 188, 122, 229, 146, 106, 127], OperandSize::Qword)
}

fn vpsraw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 225, 252], OperandSize::Dword)
}

fn vpsraw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 225, 52, 158], OperandSize::Dword)
}

fn vpsraw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 61, 161, 225, 201], OperandSize::Qword)
}

fn vpsraw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RDX, 924510338, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 162, 225, 170, 130, 232, 26, 55], OperandSize::Qword)
}

fn vpsraw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 225, 202], OperandSize::Dword)
}

fn vpsraw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 718160967, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 225, 185, 71, 68, 206, 42], OperandSize::Dword)
}

fn vpsraw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 29, 203, 225, 192], OperandSize::Qword)
}

fn vpsraw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 197, 225, 8], OperandSize::Qword)
}

