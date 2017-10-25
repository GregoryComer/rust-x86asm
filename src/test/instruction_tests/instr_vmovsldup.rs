use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 216], OperandSize::Dword)
}

#[test]
fn vmovsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 883724241, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 172, 254, 209, 143, 172, 52], OperandSize::Dword)
}

#[test]
fn vmovsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 223], OperandSize::Qword)
}

#[test]
fn vmovsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 619667705, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 36, 181, 249, 96, 239, 36], OperandSize::Qword)
}

#[test]
fn vmovsldup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 235], OperandSize::Dword)
}

#[test]
fn vmovsldup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1384310287, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 172, 250, 15, 230, 130, 82], OperandSize::Dword)
}

#[test]
fn vmovsldup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 237], OperandSize::Qword)
}

#[test]
fn vmovsldup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 36, 144], OperandSize::Qword)
}

#[test]
fn vmovsldup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 18, 249], OperandSize::Dword)
}

#[test]
fn vmovsldup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 440266077, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 18, 139, 93, 237, 61, 26], OperandSize::Dword)
}

#[test]
fn vmovsldup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 126, 138, 18, 200], OperandSize::Qword)
}

#[test]
fn vmovsldup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1384616189, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 18, 140, 137, 253, 144, 135, 82], OperandSize::Qword)
}

#[test]
fn vmovsldup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 18, 221], OperandSize::Dword)
}

#[test]
fn vmovsldup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 351409417, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 18, 132, 150, 9, 21, 242, 20], OperandSize::Dword)
}

#[test]
fn vmovsldup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 126, 175, 18, 216], OperandSize::Qword)
}

#[test]
fn vmovsldup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 173, 18, 4, 70], OperandSize::Qword)
}

#[test]
fn vmovsldup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 18, 241], OperandSize::Dword)
}

#[test]
fn vmovsldup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EAX, 686361345, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 18, 176, 1, 11, 233, 40], OperandSize::Dword)
}

#[test]
fn vmovsldup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 126, 205, 18, 220], OperandSize::Qword)
}

#[test]
fn vmovsldup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM27)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 204, 18, 24], OperandSize::Qword)
}

