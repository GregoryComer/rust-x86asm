use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu16_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 111, 216], OperandSize::Dword)
}

#[test]
fn vmovdqu16_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1767158428, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 111, 28, 253, 156, 178, 84, 105], OperandSize::Dword)
}

#[test]
fn vmovdqu16_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 255, 138, 111, 235], OperandSize::Qword)
}

#[test]
fn vmovdqu16_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 140, 111, 52, 74], OperandSize::Qword)
}

#[test]
fn vmovdqu16_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 111, 247], OperandSize::Dword)
}

#[test]
fn vmovdqu16_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 111, 26], OperandSize::Dword)
}

#[test]
fn vmovdqu16_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 173, 111, 208], OperandSize::Qword)
}

#[test]
fn vmovdqu16_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1479883548, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 255, 175, 111, 20, 245, 28, 59, 53, 88], OperandSize::Qword)
}

#[test]
fn vmovdqu16_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 201, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqu16_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 99810452, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 111, 167, 148, 252, 242, 5], OperandSize::Dword)
}

#[test]
fn vmovdqu16_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 201, 111, 225], OperandSize::Qword)
}

#[test]
fn vmovdqu16_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 255, 201, 111, 6], OperandSize::Qword)
}

#[test]
fn vmovdqu16_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 141, 111, 193], OperandSize::Dword)
}

#[test]
fn vmovdqu16_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1224182970, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 44, 133, 186, 140, 247, 72], OperandSize::Dword)
}

#[test]
fn vmovdqu16_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 255, 141, 111, 252], OperandSize::Qword)
}

#[test]
fn vmovdqu16_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 255, 8, 127, 19], OperandSize::Qword)
}

#[test]
fn vmovdqu16_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 111, 203], OperandSize::Dword)
}

#[test]
fn vmovdqu16_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1115328400, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 36, 197, 144, 143, 122, 66], OperandSize::Dword)
}

#[test]
fn vmovdqu16_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 255, 175, 111, 227], OperandSize::Qword)
}

#[test]
fn vmovdqu16_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 146886199, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 40, 127, 20, 213, 55, 78, 193, 8], OperandSize::Qword)
}

#[test]
fn vmovdqu16_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 252], OperandSize::Dword)
}

#[test]
fn vmovdqu16_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1712363166, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 180, 152, 158, 150, 16, 102], OperandSize::Dword)
}

#[test]
fn vmovdqu16_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 255, 202, 111, 252], OperandSize::Qword)
}

#[test]
fn vmovdqu16_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1935694587, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 44, 189, 251, 90, 96, 115], OperandSize::Qword)
}

