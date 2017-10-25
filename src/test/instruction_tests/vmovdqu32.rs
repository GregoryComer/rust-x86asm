use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqu32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 111, 246], OperandSize::Dword)
}

fn vmovdqu32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 2115653382, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 111, 60, 125, 6, 79, 26, 126], OperandSize::Dword)
}

fn vmovdqu32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 126, 138, 111, 217], OperandSize::Qword)
}

fn vmovdqu32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 79743919, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 111, 20, 117, 175, 203, 192, 4], OperandSize::Qword)
}

fn vmovdqu32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 111, 233], OperandSize::Dword)
}

fn vmovdqu32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 2125215501, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 111, 180, 80, 13, 55, 172, 126], OperandSize::Dword)
}

fn vmovdqu32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 126, 175, 111, 198], OperandSize::Qword)
}

fn vmovdqu32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1672891262, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 172, 111, 148, 130, 126, 75, 182, 99], OperandSize::Qword)
}

fn vmovdqu32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 111, 253], OperandSize::Dword)
}

fn vmovdqu32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1692483531, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 111, 4, 69, 203, 63, 225, 100], OperandSize::Dword)
}

fn vmovdqu32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 126, 207, 111, 227], OperandSize::Qword)
}

fn vmovdqu32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1011589979, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 204, 111, 60, 181, 91, 163, 75, 60], OperandSize::Qword)
}

fn vmovdqu32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 111, 242], OperandSize::Dword)
}

fn vmovdqu32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1150404027, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 36, 221, 187, 197, 145, 68], OperandSize::Dword)
}

fn vmovdqu32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 126, 138, 111, 206], OperandSize::Qword)
}

fn vmovdqu32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectDisplaced(RDI, 938165838, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 8, 127, 175, 78, 70, 235, 55], OperandSize::Qword)
}

fn vmovdqu32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 111, 214], OperandSize::Dword)
}

fn vmovdqu32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledDisplaced(ESI, Two, 759047945, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 28, 117, 9, 39, 62, 45], OperandSize::Dword)
}

fn vmovdqu32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 126, 175, 111, 233], OperandSize::Qword)
}

fn vmovdqu32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 18], OperandSize::Qword)
}

fn vmovdqu32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 111, 222], OperandSize::Dword)
}

fn vmovdqu32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectDisplaced(ECX, 1982576313, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 72, 127, 153, 185, 182, 43, 118], OperandSize::Dword)
}

fn vmovdqu32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 201, 111, 206], OperandSize::Qword)
}

fn vmovdqu32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectDisplaced(RSI, 296749486, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 72, 127, 166, 174, 9, 176, 17], OperandSize::Qword)
}

