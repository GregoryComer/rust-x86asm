use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 209], OperandSize::Dword)
}

fn vmovsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ESI, 777328712, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 190, 72, 24, 85, 46], OperandSize::Dword)
}

fn vmovsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 211], OperandSize::Qword)
}

fn vmovsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 28, 190], OperandSize::Qword)
}

fn vmovsldup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 219], OperandSize::Dword)
}

fn vmovsldup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EAX, 34672494, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 152, 110, 15, 17, 2], OperandSize::Dword)
}

fn vmovsldup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 239], OperandSize::Qword)
}

fn vmovsldup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 20, 128], OperandSize::Qword)
}

fn vmovsldup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 18, 241], OperandSize::Dword)
}

fn vmovsldup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 652653201, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 18, 183, 145, 178, 230, 38], OperandSize::Dword)
}

fn vmovsldup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 126, 142, 18, 194], OperandSize::Qword)
}

fn vmovsldup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 1774502064, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 18, 151, 176, 192, 196, 105], OperandSize::Qword)
}

fn vmovsldup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 18, 254], OperandSize::Dword)
}

fn vmovsldup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 18, 14], OperandSize::Dword)
}

fn vmovsldup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 18, 192], OperandSize::Qword)
}

fn vmovsldup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1623913849, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 126, 169, 18, 188, 247, 121, 245, 202, 96], OperandSize::Qword)
}

fn vmovsldup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 18, 212], OperandSize::Dword)
}

fn vmovsldup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1711554631, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 18, 132, 195, 71, 64, 4, 102], OperandSize::Dword)
}

fn vmovsldup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 126, 203, 18, 230], OperandSize::Qword)
}

fn vmovsldup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 573809148, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 203, 18, 60, 69, 252, 161, 51, 34], OperandSize::Qword)
}

