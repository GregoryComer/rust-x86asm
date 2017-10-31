use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 234], OperandSize::Dword)
}

#[test]
fn vmovddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1746290603, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 172, 151, 171, 71, 22, 104], OperandSize::Dword)
}

#[test]
fn vmovddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 198], OperandSize::Qword)
}

#[test]
fn vmovddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1073545216, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 44, 117, 0, 0, 253, 63], OperandSize::Qword)
}

#[test]
fn vmovddup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 219], OperandSize::Dword)
}

#[test]
fn vmovddup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1401441138, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 188, 88, 114, 75, 136, 83], OperandSize::Dword)
}

#[test]
fn vmovddup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 220], OperandSize::Qword)
}

#[test]
fn vmovddup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1421462804, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 180, 241, 20, 205, 185, 84], OperandSize::Qword)
}

#[test]
fn vmovddup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 18, 231], OperandSize::Dword)
}

#[test]
fn vmovddup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ECX, 1204718342, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 18, 169, 6, 139, 206, 71], OperandSize::Dword)
}

#[test]
fn vmovddup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 142, 18, 196], OperandSize::Qword)
}

#[test]
fn vmovddup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 227220393, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 18, 36, 189, 169, 27, 139, 13], OperandSize::Qword)
}

#[test]
fn vmovddup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 18, 204], OperandSize::Dword)
}

#[test]
fn vmovddup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 169, 18, 30], OperandSize::Dword)
}

#[test]
fn vmovddup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 172, 18, 226], OperandSize::Qword)
}

#[test]
fn vmovddup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 874696525, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 170, 18, 52, 197, 77, 207, 34, 52], OperandSize::Qword)
}

#[test]
fn vmovddup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 203, 18, 199], OperandSize::Dword)
}

#[test]
fn vmovddup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 390415536, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 201, 18, 148, 118, 176, 68, 69, 23], OperandSize::Dword)
}

#[test]
fn vmovddup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 255, 207, 18, 239], OperandSize::Qword)
}

#[test]
fn vmovddup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 205, 18, 16], OperandSize::Qword)
}

