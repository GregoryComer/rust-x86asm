use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 244], OperandSize::Dword)
}

#[test]
fn vmovsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 52, 75], OperandSize::Dword)
}

#[test]
fn vmovsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 232], OperandSize::Qword)
}

#[test]
fn vmovsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 1637318368, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 138, 224, 126, 151, 97], OperandSize::Qword)
}

#[test]
fn vmovsldup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 209], OperandSize::Dword)
}

#[test]
fn vmovsldup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EBX, 975308629, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 155, 85, 7, 34, 58], OperandSize::Dword)
}

#[test]
fn vmovsldup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 251], OperandSize::Qword)
}

#[test]
fn vmovsldup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 58], OperandSize::Qword)
}

#[test]
fn vmovsldup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 18, 245], OperandSize::Dword)
}

#[test]
fn vmovsldup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1763426674, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 18, 164, 247, 114, 193, 27, 105], OperandSize::Dword)
}

#[test]
fn vmovsldup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 138, 18, 192], OperandSize::Qword)
}

#[test]
fn vmovsldup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2040416261, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 142, 18, 12, 69, 5, 72, 158, 121], OperandSize::Qword)
}

#[test]
fn vmovsldup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 18, 215], OperandSize::Dword)
}

#[test]
fn vmovsldup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 18, 20, 137], OperandSize::Dword)
}

#[test]
fn vmovsldup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 126, 172, 18, 223], OperandSize::Qword)
}

#[test]
fn vmovsldup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM9)), operand2: Some(IndirectDisplaced(RBX, 1690521881, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 170, 18, 139, 25, 81, 195, 100], OperandSize::Qword)
}

#[test]
fn vmovsldup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 18, 220], OperandSize::Dword)
}

#[test]
fn vmovsldup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 18, 12, 78], OperandSize::Dword)
}

#[test]
fn vmovsldup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 126, 201, 18, 227], OperandSize::Qword)
}

#[test]
fn vmovsldup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM25)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 205, 18, 14], OperandSize::Qword)
}

